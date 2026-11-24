use std::time::Duration;

use rust_decimal::Decimal;
use thiserror::Error;

use crate::{
    MAX_PRICE_RESPONSE_BYTES, PriceProvider, PriceResponseError, cart::valid_product,
    parse_price_response,
};

const PRICE_API_URL: &str = "https://equalexperts.github.io/backend-take-home-test-data";

/// Retrieves ordinary decimal JSON prices, with a five-second request budget.
/// Responses larger than 16 KiB and exponent-form prices are rejected.
pub struct HttpPriceClient {
    agent: ureq::Agent,
    base_url: String,
}

impl HttpPriceClient {
    /// Uses the price service supplied with the assignment.
    pub fn new() -> Self {
        Self::with_base_url(PRICE_API_URL)
    }

    /// Overrides the base URL, for example with a local fixture server.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .http_status_as_error(false)
            .build();
        Self {
            agent: config.into(),
            base_url: base_url.into().trim_end_matches('/').to_owned(),
        }
    }
}

impl Default for HttpPriceClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
pub enum PriceClientError {
    #[error("product must be a lowercase ASCII slug: {0:?}")]
    InvalidProduct(String),
    #[error("product not found: {0}")]
    NotFound(String),
    #[error("price request failed: {0}")]
    Request(#[source] ureq::Error),
    #[error("price service returned HTTP {0}")]
    HttpStatus(u16),
    #[error("could not read price response: {0}")]
    Response(#[source] ureq::Error),
    #[error(
        "invalid price {0}: expected an ordinary nonnegative decimal in exact cents within range"
    )]
    InvalidPrice(String),
}

impl PriceProvider for HttpPriceClient {
    type Error = PriceClientError;

    fn unit_price(&self, product: &str) -> Result<Decimal, Self::Error> {
        if !valid_product(product) {
            return Err(PriceClientError::InvalidProduct(product.to_owned()));
        }

        let url = format!("{}/{product}.json", self.base_url);
        let mut response = self
            .agent
            .get(&url)
            .header("Accept", "application/json")
            .call()
            .map_err(PriceClientError::Request)?;
        if response.status() == 404 {
            return Err(PriceClientError::NotFound(product.to_owned()));
        }
        if !response.status().is_success() {
            return Err(PriceClientError::HttpStatus(response.status().as_u16()));
        }

        let body = response
            .body_mut()
            .with_config()
            // ureq needs one byte of lookahead to observe EOF at the accepted limit.
            .limit(MAX_PRICE_RESPONSE_BYTES as u64 + 1)
            .lossy_utf8(false)
            .read_to_string()
            .map_err(PriceClientError::Response)?;
        parse_price_response(&body).map_err(|error| match error {
            PriceResponseError::InvalidResponse(error) => PriceClientError::Response(error.into()),
            PriceResponseError::InvalidPrice(value) => PriceClientError::InvalidPrice(value),
        })
    }
}
