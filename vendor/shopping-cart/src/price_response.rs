use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::value::RawValue;
use thiserror::Error;

use crate::cart::price_in_cents;

pub const MAX_PRICE_RESPONSE_BYTES: usize = 16 * 1024;

#[derive(Debug, Error)]
pub enum PriceResponseError {
    #[error("invalid price response: {0}")]
    InvalidResponse(#[source] serde_json::Error),
    #[error(
        "invalid price {0}: expected an ordinary nonnegative decimal in exact cents within range"
    )]
    InvalidPrice(String),
}

#[derive(Deserialize)]
struct PriceResponse {
    price: Box<RawValue>,
}

/// Parses a bounded JSON object containing an ordinary decimal numeric `price`.
/// Significant fractional cents, exponent notation and out-of-range values fail.
/// Callers must bound network reads before allocating the response body.
pub fn parse_price_response(body: &str) -> Result<Decimal, PriceResponseError> {
    if body.len() > MAX_PRICE_RESPONSE_BYTES {
        return Err(response_shape_error("response exceeds 16 KiB"));
    }
    if !body.trim_start().starts_with('{') {
        return Err(response_shape_error("expected a JSON object"));
    }
    let response: PriceResponse =
        serde_json::from_str(body).map_err(PriceResponseError::InvalidResponse)?;
    let raw_price = response.price.get().trim();
    // RawValue has validated JSON syntax, so the first byte identifies a number token.
    if !raw_price
        .bytes()
        .next()
        .is_some_and(|byte| byte == b'-' || byte.is_ascii_digit())
    {
        return Err(response_shape_error("price must be a JSON number"));
    }
    let price = Decimal::from_str_exact(raw_price)
        .map_err(|_| PriceResponseError::InvalidPrice(raw_price.to_owned()))?;
    if price_in_cents(price).is_none() {
        return Err(PriceResponseError::InvalidPrice(raw_price.to_owned()));
    }
    Ok(price)
}

fn response_shape_error(message: &str) -> PriceResponseError {
    let error = <serde_json::Error as serde::de::Error>::custom(message);
    PriceResponseError::InvalidResponse(error)
}
