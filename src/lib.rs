//! Exact cart behavior in the browser, using the original Rust library.
//!
//! Strings preserve decimal text and prevent JavaScript-to-u32 coercion.

use std::{cell::Cell, convert::Infallible, rc::Rc};

use equal_experts_shopping_cart::{
    CartSnapshot, PriceProvider, ShoppingCart, parse_price_response,
};
use rust_decimal::Decimal;
use serde::Serialize;
use wasm_bindgen::prelude::*;

struct BrowserPrice(Rc<Cell<Decimal>>);

impl PriceProvider for BrowserPrice {
    type Error = Infallible;

    fn unit_price(&self, _: &str) -> Result<Decimal, Self::Error> {
        Ok(self.0.get())
    }
}

/// An independent in-memory cart. All exported money values are decimal strings.
#[wasm_bindgen]
pub struct BrowserCart {
    price: Rc<Cell<Decimal>>,
    cart: ShoppingCart<BrowserPrice>,
}

impl Default for BrowserCart {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl BrowserCart {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let price = Rc::new(Cell::new(Decimal::ZERO));
        Self {
            cart: ShoppingCart::new(BrowserPrice(Rc::clone(&price))),
            price,
        }
    }

    /// Applies one raw API response. A rejected addition preserves cart state.
    pub fn add(&mut self, product: &str, quantity: &str, response: &str) -> String {
        let result = self.try_add(product, quantity, response);
        serde_json::json!({
            "ok": result.is_ok(),
            "error": result.err(),
            "snapshot": view(self.cart.snapshot()),
        })
        .to_string()
    }

    pub fn snapshot(&self) -> String {
        serde_json::json!(view(self.cart.snapshot())).to_string()
    }

    /// Starts a new local session; no server state or persistence exists.
    pub fn reset(&mut self) -> String {
        self.cart = ShoppingCart::new(BrowserPrice(Rc::clone(&self.price)));
        self.snapshot()
    }
}

impl BrowserCart {
    fn try_add(&mut self, product: &str, quantity: &str, response: &str) -> Result<(), String> {
        if quantity.is_empty() || !quantity.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err("quantity must contain only decimal digits".to_owned());
        }
        let quantity = quantity
            .parse::<u32>()
            .map_err(|_| "quantity must fit a 32-bit unsigned integer".to_owned())?;
        let price = parse_price_response(response).map_err(|error| error.to_string())?;
        self.price.set(price);
        self.cart
            .add(product, quantity)
            .map_err(|error| error.to_string())
    }
}

#[derive(Serialize)]
struct ItemView {
    product: String,
    quantity: u32,
    unit_price: String,
    line_total: String,
}

#[derive(Serialize)]
struct SnapshotView {
    items: Vec<ItemView>,
    subtotal: String,
    tax: String,
    total: String,
}

fn view(snapshot: CartSnapshot) -> SnapshotView {
    SnapshotView {
        items: snapshot
            .items
            .into_iter()
            .map(|(product, item)| ItemView {
                product,
                quantity: item.quantity,
                unit_price: format!("{:.2}", item.unit_price),
                // The core snapshot already guarantees this line fits u64 cents.
                line_total: format!("{:.2}", item.unit_price * Decimal::from(item.quantity)),
            })
            .collect(),
        subtotal: format!("{:.2}", snapshot.subtotal),
        tax: format!("{:.2}", snapshot.tax),
        total: format!("{:.2}", snapshot.total),
    }
}
