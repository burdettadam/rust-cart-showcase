//! Shopping-cart library for the Equal Experts assignment.
//!
//! The price boundary can be replaced with a deterministic provider in tests.
//!
//! ```
//! use equal_experts_shopping_cart::{PriceProvider, ShoppingCart};
//! use rust_decimal::Decimal;
//! use std::convert::Infallible;
//!
//! struct FixedPrices;
//! impl PriceProvider for FixedPrices {
//!     type Error = Infallible;
//!
//!     fn unit_price(&self, _: &str) -> Result<Decimal, Self::Error> {
//!         Ok(Decimal::new(252, 2))
//!     }
//! }
//!
//! let mut cart = ShoppingCart::new(FixedPrices);
//! cart.add("cornflakes", 2)?;
//! assert_eq!(cart.snapshot().total, Decimal::new(567, 2));
//! # Ok::<(), equal_experts_shopping_cart::CartError<Infallible>>(())
//! ```

mod cart;
#[cfg(feature = "native-http")]
mod http_price_client;
mod price_response;

pub use cart::{CartError, CartItem, CartSnapshot, PriceProvider, ShoppingCart};
#[cfg(feature = "native-http")]
pub use http_price_client::{HttpPriceClient, PriceClientError};
pub use price_response::{MAX_PRICE_RESPONSE_BYTES, PriceResponseError, parse_price_response};
