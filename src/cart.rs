use std::collections::BTreeMap;

use rust_decimal::{Decimal, RoundingStrategy};
use thiserror::Error;

/// Retrieves a current unit price without changing the cart.
pub trait PriceProvider {
    type Error: std::error::Error + 'static;

    fn unit_price(&self, product: &str) -> Result<Decimal, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartItem {
    pub quantity: u32,
    pub unit_price: Decimal,
}

/// An owned view; changing it cannot change the cart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartSnapshot {
    pub items: BTreeMap<String, CartItem>,
    pub subtotal: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
}

/// Receipt details for the last successful addition. The snapshot subtotal is taxable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartReceipt {
    pub snapshot: CartSnapshot,
    pub merchandise_subtotal: Decimal,
    pub discount: Decimal,
    /// Products with at least one unit awarded by the promotion.
    pub free_quantities: BTreeMap<String, u32>,
}

#[derive(Debug, Error)]
pub enum CartError<E: std::error::Error + 'static> {
    #[error("quantity must be a positive integer")]
    InvalidQuantity,
    #[error("product must be a lowercase ASCII slug: {0:?}")]
    InvalidProduct(String),
    #[error("invalid unit price {0}: expected a nonnegative amount in exact cents within range")]
    InvalidPrice(Decimal),
    #[error("combined quantity exceeds u32::MAX")]
    QuantityOverflow,
    #[error("cart amounts exceed the supported maximum of 184467440737095516.15")]
    AmountOverflow,
    #[error("price lookup failed: {0}")]
    PriceLookup(#[source] E),
}

/// A synchronous cart with exact cents, bounded by `u64::MAX` cents per amount.
pub struct ShoppingCart<P: PriceProvider> {
    prices: P,
    receipt: CartReceipt,
    every_third_free: bool,
}

impl<P: PriceProvider> ShoppingCart<P> {
    pub fn new(prices: P) -> Self {
        Self {
            prices,
            receipt: CartReceipt {
                snapshot: CartSnapshot {
                    items: BTreeMap::new(),
                    subtotal: Decimal::new(0, 2),
                    tax: Decimal::new(0, 2),
                    total: Decimal::new(0, 2),
                },
                merchandise_subtotal: Decimal::new(0, 2),
                discount: Decimal::new(0, 2),
                free_quantities: BTreeMap::new(),
            },
            every_third_free: false,
        }
    }

    /// Every third successfully added unit is free, across products and calls.
    pub fn with_every_third_free(prices: P) -> Self {
        Self {
            every_third_free: true,
            ..Self::new(prices)
        }
    }

    /// Adds quantity using the fetched price for every unit of this product.
    /// Any error leaves the previous cart state unchanged.
    pub fn add(&mut self, product: &str, quantity: u32) -> Result<(), CartError<P::Error>> {
        if quantity == 0 {
            return Err(CartError::InvalidQuantity);
        }
        if !valid_product(product) {
            return Err(CartError::InvalidProduct(product.to_owned()));
        }

        let combined_quantity = self
            .receipt
            .snapshot
            .items
            .get(product)
            .map_or(0, |item| item.quantity)
            .checked_add(quantity)
            .ok_or(CartError::QuantityOverflow)?;
        let price = self
            .prices
            .unit_price(product)
            .map_err(CartError::PriceLookup)?;
        let unit_price = checked_money(price).ok_or(CartError::InvalidPrice(price))?;

        let mut free_quantities = self.receipt.free_quantities.clone();
        let new_free_units = self.free_units_for_addition(quantity);
        if new_free_units > 0 {
            *free_quantities.entry(product.to_owned()).or_default() += new_free_units;
        }
        let mut items = self.receipt.snapshot.items.clone();
        items.insert(
            product.to_owned(),
            CartItem {
                quantity: combined_quantity,
                unit_price,
            },
        );
        self.receipt =
            calculate_receipt(items, free_quantities).ok_or(CartError::AmountOverflow)?;
        Ok(())
    }

    fn free_units_for_addition(&self, quantity: u32) -> u32 {
        if !self.every_third_free {
            return 0;
        }

        let units_in_cart: u128 = self
            .receipt
            .snapshot
            .items
            .values()
            .map(|item| u128::from(item.quantity))
            .sum();
        let groups_before = units_in_cart / 3;
        let groups_after = (units_in_cart + u128::from(quantity)) / 3;
        (groups_after - groups_before) as u32
    }

    /// Returns the last validated state without retrieving prices again.
    pub fn snapshot(&self) -> CartSnapshot {
        self.receipt.snapshot.clone()
    }

    /// Returns totals and the awarded quantities without retrieving prices again.
    pub fn receipt(&self) -> CartReceipt {
        self.receipt.clone()
    }
}

pub(crate) fn valid_product(product: &str) -> bool {
    product.split('-').all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    })
}

pub(crate) fn checked_money(amount: Decimal) -> Option<Decimal> {
    let maximum = Decimal::from_i128_with_scale(18_446_744_073_709_551_615, 2);
    let mut amount = amount.normalize();
    if amount < Decimal::ZERO || amount > maximum || amount.scale() > 2 {
        return None;
    }
    amount.rescale(2);
    Some(amount)
}

fn calculate_receipt(
    items: BTreeMap<String, CartItem>,
    free_quantities: BTreeMap<String, u32>,
) -> Option<CartReceipt> {
    let mut merchandise = Decimal::new(0, 2);
    let mut discount = Decimal::new(0, 2);
    for (product, item) in &items {
        let line_total = item.unit_price.checked_mul(Decimal::from(item.quantity))?;
        merchandise = checked_money(merchandise.checked_add(line_total)?)?;
        let free_quantity = free_quantities.get(product).copied().unwrap_or(0);
        let line_discount = item.unit_price.checked_mul(Decimal::from(free_quantity))?;
        discount = discount.checked_add(line_discount)?;
    }
    let subtotal = merchandise.checked_sub(discount)?;
    let tax = subtotal
        .checked_mul(Decimal::new(125, 3))?
        .round_dp_with_strategy(2, RoundingStrategy::ToPositiveInfinity);
    let total = checked_money(subtotal.checked_add(tax)?)?;
    Some(CartReceipt {
        snapshot: CartSnapshot {
            items,
            subtotal,
            tax,
            total,
        },
        merchandise_subtotal: merchandise,
        discount,
        free_quantities,
    })
}
