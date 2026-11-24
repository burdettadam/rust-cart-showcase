use std::error::Error;

use equal_experts_shopping_cart::{
    CartError, HttpPriceClient, PriceClientError, PriceProvider, ShoppingCart,
};
use rust_decimal::Decimal;

fn decimal(value: &str) -> Decimal {
    Decimal::from_str_exact(value).unwrap()
}

#[test]
fn gets_the_expected_json_path_under_the_configured_base_url() {
    let mut server = mockito::Server::new();
    let request = server
        .mock("GET", "/prices/cornflakes.json")
        .match_header("accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"title":"Corn Flakes","price":2.52}"#)
        .create();
    let client = HttpPriceClient::with_base_url(format!("{}/prices/", server.url()));

    let price = client.unit_price("cornflakes").unwrap();

    assert_eq!(price, decimal("2.52"));
    request.assert();
}

#[test]
fn preserves_exact_cents_at_a_value_binary_floating_point_cannot_represent() {
    let mut server = mockito::Server::new();
    let request = server
        .mock("GET", "/cornflakes.json")
        .with_body(r#"{"price":184467440737095516.15}"#)
        .create();
    let client = HttpPriceClient::with_base_url(server.url());

    let price = client.unit_price("cornflakes").unwrap();

    assert_eq!(price.to_string(), "184467440737095516.15");
    request.assert();
}

#[test]
fn accepts_free_whole_number_and_trailing_zero_prices() {
    for (literal, expected) in [
        ("0", "0.00"),
        ("2", "2.00"),
        ("2.5200", "2.52"),
        ("2.52000000000000000000000000000", "2.52"),
    ] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_body(format!(r#"{{"price":{literal}}}"#))
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        assert_eq!(client.unit_price("cornflakes").unwrap(), decimal(expected));
        request.assert();
    }
}

#[test]
fn reports_an_unknown_product_without_trying_to_parse_its_error_page() {
    let mut server = mockito::Server::new();
    let request = server
        .mock("GET", "/unknown-product.json")
        .with_status(404)
        .with_body("<html>Not Found</html>")
        .create();
    let client = HttpPriceClient::with_base_url(server.url());

    let error = client.unit_price("unknown-product").unwrap_err();

    assert!(matches!(&error, PriceClientError::NotFound(product) if product == "unknown-product"));
    assert!(error.to_string().contains("unknown-product"));
    request.assert();
}

#[test]
fn rejects_unsuccessful_http_statuses_even_when_the_body_has_a_price() {
    for status in [304, 401, 500] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_status(status)
            .with_body(r#"{"price":2.52}"#)
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        let error = client.unit_price("cornflakes").unwrap_err();

        assert!(
            matches!(&error, PriceClientError::HttpStatus(actual) if usize::from(*actual) == status)
        );
        assert!(error.to_string().contains(&status.to_string()));
        request.assert();
    }
}

#[test]
fn rejects_malformed_json_and_missing_or_non_numeric_price_fields() {
    for body in [
        "not JSON",
        "{}",
        r#"{"price":null}"#,
        r#"{"price":true}"#,
        r#"{"price":"2.52"}"#,
        r#"{"price":[]}"#,
        r#"{"price":2.52} trailing garbage"#,
        "\u{feff}{\"price\":2.52}",
    ] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_body(body)
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        let error = client.unit_price("cornflakes").unwrap_err();

        assert!(
            matches!(&error, PriceClientError::Response(_)),
            "{body}: {error}"
        );
        assert!(error.source().is_some());
        assert!(error.to_string().contains("price response"));
        request.assert();
    }
}

#[test]
fn rejects_invalid_utf8_in_otherwise_valid_response_metadata() {
    for content_type in ["application/json", "text/plain; charset=utf-8"] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_header("content-type", content_type)
            .with_body(b"{\"title\":\"\xff\",\"price\":2.52}")
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        let error = client.unit_price("cornflakes").unwrap_err();

        assert!(matches!(&error, PriceClientError::Response(_)));
        assert!(error.source().is_some());
        request.assert();
    }
}

#[test]
fn response_shape_requires_a_top_level_object() {
    let mut server = mockito::Server::new();
    let request = server
        .mock("GET", "/cornflakes.json")
        .with_body("[2.52]")
        .create();
    let client = HttpPriceClient::with_base_url(server.url());

    let error = client.unit_price("cornflakes").unwrap_err();

    assert!(matches!(error, PriceClientError::Response(_)));
    request.assert();
}

#[test]
fn response_shape_rejects_objects_that_mimic_serde_private_tokens() {
    for body in [
        r#"{"price":{"$serde_json::private::Number":"2.52"}}"#,
        r#"{"price":{"$serde_json::private::RawValue":"2.52"}}"#,
    ] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_body(body)
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        let error = client.unit_price("cornflakes").unwrap_err();

        assert!(matches!(error, PriceClientError::Response(_)));
        request.assert();
    }
}

#[test]
fn response_shape_errors_leave_an_existing_cart_unchanged() {
    for body in [
        "[2.52]",
        r#"{"price":{"$serde_json::private::Number":"2.52"}}"#,
    ] {
        let mut server = mockito::Server::new();
        let cornflakes = server
            .mock("GET", "/cornflakes.json")
            .with_body(r#"{"price":2.52}"#)
            .create();
        let weetabix = server
            .mock("GET", "/weetabix.json")
            .with_body(body)
            .create();
        let mut cart = ShoppingCart::new(HttpPriceClient::with_base_url(server.url()));
        cart.add("cornflakes", 1).unwrap();
        let before = cart.snapshot();

        let error = cart.add("weetabix", 2).unwrap_err();

        assert!(matches!(
            error,
            CartError::PriceLookup(PriceClientError::Response(_))
        ));
        assert_eq!(cart.snapshot(), before);
        cornflakes.assert();
        weetabix.assert();
    }
}

#[test]
fn rejects_negative_fractional_cent_and_out_of_range_prices() {
    for literal in [
        "-0.01",
        "1.001",
        "184467440737095516.16",
        "184467440737095517",
        "79228162514264337593543950336",
    ] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_body(format!(r#"{{"price":{literal}}}"#))
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        let error = client.unit_price("cornflakes").unwrap_err();

        assert!(matches!(&error, PriceClientError::InvalidPrice(value) if value == literal));
        assert!(error.to_string().contains("exact cents"));
        request.assert();
    }
}

#[test]
fn invalid_prices_leave_an_existing_cart_unchanged() {
    for literal in [
        "1.0010000000000000000000000000000000000000",
        "184467440737095516.1600000000000000000000",
        "2.52000000000000000000000000000e0",
    ] {
        let mut server = mockito::Server::new();
        let cornflakes = server
            .mock("GET", "/cornflakes.json")
            .with_body(r#"{"price":2.52}"#)
            .create();
        let weetabix = server
            .mock("GET", "/weetabix.json")
            .with_body(format!(r#"{{"price":{literal}}}"#))
            .create();
        let mut cart = ShoppingCart::new(HttpPriceClient::with_base_url(server.url()));
        cart.add("cornflakes", 1).unwrap();
        let before = cart.snapshot();

        let error = cart.add("weetabix", 2).unwrap_err();

        assert!(matches!(
            error,
            CartError::PriceLookup(PriceClientError::InvalidPrice(value)) if value == literal
        ));
        assert_eq!(cart.snapshot(), before);
        cornflakes.assert();
        weetabix.assert();
    }
}

#[test]
fn rejects_excess_precision_that_would_round_into_an_apparently_valid_cent() {
    let mut server = mockito::Server::new();
    let request = server
        .mock("GET", "/cornflakes.json")
        .with_body(r#"{"price":0.01000000000000000000000000001}"#)
        .create();
    let client = HttpPriceClient::with_base_url(server.url());

    let error = client.unit_price("cornflakes").unwrap_err();

    assert!(matches!(error, PriceClientError::InvalidPrice(_)));
    request.assert();
}

#[test]
fn explicitly_rejects_scientific_notation_instead_of_using_a_lossy_fallback() {
    for literal in ["2.52e0", "252e-2", "1e-29"] {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/cornflakes.json")
            .with_body(format!(r#"{{"price":{literal}}}"#))
            .create();
        let client = HttpPriceClient::with_base_url(server.url());

        let error = client.unit_price("cornflakes").unwrap_err();

        assert!(matches!(error, PriceClientError::InvalidPrice(_)));
        request.assert();
    }
}

#[test]
fn rejects_invalid_product_identifiers_before_sending_http() {
    let mut server = mockito::Server::new();
    let request = server.mock("GET", mockito::Matcher::Any).expect(0).create();
    let client = HttpPriceClient::with_base_url(server.url());

    let error = client.unit_price("../cornflakes").unwrap_err();

    assert!(
        matches!(&error, PriceClientError::InvalidProduct(product) if product == "../cornflakes")
    );
    assert!(error.to_string().contains("lowercase ASCII slug"));
    request.assert();
}

#[test]
fn malformed_base_url_returns_a_request_error_with_its_source() {
    let client = HttpPriceClient::with_base_url("::not-a-url");

    let error = client.unit_price("cornflakes").unwrap_err();

    assert!(matches!(&error, PriceClientError::Request(_)));
    assert!(error.source().is_some());
    assert!(error.to_string().contains("price request failed"));
}

#[test]
fn accepts_exact_response_byte_limit_and_rejects_one_more() {
    let document = r#"{"price":2.52}"#;
    for size in [16 * 1024 - 1, 16 * 1024, 16 * 1024 + 1] {
        for leading in [false, true] {
            let padding = " ".repeat(size - document.len());
            let body = if leading {
                format!("{padding}{document}")
            } else {
                format!("{document}{padding}")
            };
            let mut server = mockito::Server::new();
            let request = server
                .mock("GET", "/cornflakes.json")
                .with_body(body)
                .create();
            let client = HttpPriceClient::with_base_url(server.url());
            let result = client.unit_price("cornflakes");

            if size <= 16 * 1024 {
                assert_eq!(result.unwrap(), Decimal::new(252, 2));
            } else {
                assert!(matches!(result, Err(PriceClientError::Response(_))));
            }
            request.assert();
        }
    }
}

#[test]
fn rejects_an_oversized_response_body() {
    let mut server = mockito::Server::new();
    let request = server
        .mock("GET", "/cornflakes.json")
        .with_body(format!("{}{{\"price\":2.52}}", " ".repeat(20_000)))
        .create();
    let client = HttpPriceClient::with_base_url(server.url());

    let error = client.unit_price("cornflakes").unwrap_err();

    assert!(matches!(&error, PriceClientError::Response(_)));
    assert!(error.source().is_some());
    request.assert();
}

#[test]
fn complete_assignment_example_uses_one_request_per_add_and_no_requests_on_reads() {
    let mut server = mockito::Server::new();
    let cornflakes = server
        .mock("GET", "/cornflakes.json")
        .with_body(r#"{"title":"Corn Flakes","price":2.52}"#)
        .expect(2)
        .create();
    let weetabix = server
        .mock("GET", "/weetabix.json")
        .with_body(r#"{"title":"Weetabix","price":9.98}"#)
        .expect(1)
        .create();
    let mut cart = ShoppingCart::new(HttpPriceClient::with_base_url(server.url()));

    cart.add("cornflakes", 1).unwrap();
    cart.add("cornflakes", 1).unwrap();
    cart.add("weetabix", 1).unwrap();

    let state = cart.snapshot();
    assert_eq!(state.items["cornflakes"].quantity, 2);
    assert_eq!(state.items["weetabix"].quantity, 1);
    assert_eq!(state.subtotal, decimal("15.02"));
    assert_eq!(state.tax, decimal("1.88"));
    assert_eq!(state.total, decimal("16.90"));
    assert_eq!(cart.snapshot(), state);
    cornflakes.assert();
    weetabix.assert();
}

#[test]
fn actual_http_failure_does_not_change_an_existing_cart() {
    let mut server = mockito::Server::new();
    let cornflakes = server
        .mock("GET", "/cornflakes.json")
        .with_body(r#"{"price":2.52}"#)
        .create();
    let weetabix = server
        .mock("GET", "/weetabix.json")
        .with_status(500)
        .create();
    let mut cart = ShoppingCart::new(HttpPriceClient::with_base_url(server.url()));
    cart.add("cornflakes", 1).unwrap();
    let before = cart.snapshot();

    let error = cart.add("weetabix", 1).unwrap_err();

    assert!(matches!(
        error,
        CartError::PriceLookup(PriceClientError::HttpStatus(500))
    ));
    assert_eq!(cart.snapshot(), before);
    cornflakes.assert();
    weetabix.assert();
}

#[test]
fn interrupted_response_body_does_not_change_an_existing_cart() {
    let mut server = mockito::Server::new();
    let cornflakes = server
        .mock("GET", "/cornflakes.json")
        .with_body(r#"{"price":2.52}"#)
        .create();
    let weetabix = server
        .mock("GET", "/weetabix.json")
        .with_chunked_body(|writer| {
            writer.write_all(br#"{"price":9.98}"#)?;
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "fixture interrupted its response",
            ))
        })
        .create();
    let mut cart = ShoppingCart::new(HttpPriceClient::with_base_url(server.url()));
    cart.add("cornflakes", 1).unwrap();
    let before = cart.snapshot();

    let error = cart.add("weetabix", 1).unwrap_err();

    assert!(matches!(
        &error,
        CartError::PriceLookup(PriceClientError::Response(_))
    ));
    assert!(error.source().is_some());
    assert_eq!(cart.snapshot(), before);
    cornflakes.assert();
    weetabix.assert();
}
