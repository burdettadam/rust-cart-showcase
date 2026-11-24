use std::collections::BTreeMap;

use rust_decimal::Decimal;
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
    state: CartSnapshot,
}

impl<P: PriceProvider> ShoppingCart<P> {
    pub fn new(prices: P) -> Self {
        Self {
            prices,
            state: CartSnapshot {
                items: BTreeMap::new(),
                subtotal: money(0),
                tax: money(0),
                total: money(0),
            },
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

        let quantity = self
            .state
            .items
            .get(product)
            .map_or(0, |item| item.quantity)
            .checked_add(quantity)
            .ok_or(CartError::QuantityOverflow)?;
        let price = self
            .prices
            .unit_price(product)
            .map_err(CartError::PriceLookup)?;
        let cents = price_in_cents(price).ok_or(CartError::InvalidPrice(price))?;

        let mut items = self.state.items.clone();
        items.insert(
            product.to_owned(),
            CartItem {
                quantity,
                unit_price: money(cents),
            },
        );
        let next = calculate_snapshot(items).ok_or(CartError::AmountOverflow)?;
        self.state = next;
        Ok(())
    }

    /// Returns the last validated state without retrieving prices again.
    pub fn snapshot(&self) -> CartSnapshot {
        self.state.clone()
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

pub(crate) fn price_in_cents(price: Decimal) -> Option<u64> {
    if price < Decimal::ZERO {
        return None;
    }
    let price = price.normalize();
    if price.scale() > 2 {
        return None;
    }
    u64::try_from(price.mantissa())
        .ok()?
        .checked_mul(10_u64.pow(2 - price.scale()))
}

fn calculate_snapshot(items: BTreeMap<String, CartItem>) -> Option<CartSnapshot> {
    let subtotal = items.values().try_fold(0_u64, |subtotal, item| {
        let line = price_in_cents(item.unit_price)?.checked_mul(u64::from(item.quantity))?;
        subtotal.checked_add(line)
    })?;
    let tax = subtotal / 8 + u64::from(subtotal % 8 != 0);
    let total = subtotal.checked_add(tax)?;
    Some(CartSnapshot {
        items,
        subtotal: money(subtotal),
        tax: money(tax),
        total: money(total),
    })
}

fn money(cents: u64) -> Decimal {
    // A u64 coefficient always fits Decimal's 96-bit mantissa.
    Decimal::from_i128_with_scale(i128::from(cents), 2)
}
