use std::error::Error;

use equal_experts_shopping_cart::{
    MAX_PRICE_RESPONSE_BYTES, PriceResponseError, parse_price_response,
};
use rust_decimal::Decimal;

#[test]
fn accepts_exact_prices_without_binary_float_conversion() {
    for (literal, expected) in [
        ("0", "0"),
        ("-0.00", "0"),
        ("2", "2"),
        ("1200", "1200"),
        ("2.5200", "2.52"),
        ("184467440737095516.15", "184467440737095516.15"),
    ] {
        let body = format!(r#"{{"price":{literal}}}"#);
        let price = parse_price_response(&body).unwrap();

        assert_eq!(price, Decimal::from_str_exact(expected).unwrap());
    }
}

#[test]
fn accepts_insignificant_zeros_beyond_decimal_representation_limits() {
    for (literal, expected) in [
        (format!("2.52{}", "0".repeat(27)), "2.52"),
        (format!("0.{}", "0".repeat(128)), "0"),
        (format!("-0.{}", "0".repeat(128)), "0"),
        (
            format!("184467440737095516.15{}", "0".repeat(128)),
            "184467440737095516.15",
        ),
    ] {
        let body = format!(r#"{{"price":{literal}}}"#);

        assert_eq!(
            parse_price_response(&body).unwrap(),
            Decimal::from_str_exact(expected).unwrap()
        );
    }
}

#[test]
fn bounds_the_original_body_before_ignoring_fractional_zeros() {
    let document = r#"{"price":2.52}"#;
    let zeros = "0".repeat(MAX_PRICE_RESPONSE_BYTES - document.len());
    let body = format!(r#"{{"price":2.52{zeros}}}"#);

    assert_eq!(body.len(), MAX_PRICE_RESPONSE_BYTES);
    assert_eq!(parse_price_response(&body).unwrap(), Decimal::new(252, 2));

    let oversized = format!(r#"{{"price":2.52{zeros}0}}"#);
    let error = parse_price_response(&oversized).unwrap_err();
    assert!(matches!(&error, PriceResponseError::InvalidResponse(_)));
    assert!(error.to_string().contains("16 KiB"));
}

#[test]
fn allows_whitespace_and_additional_api_metadata() {
    let body = " \n{\"title\":\"Corn Flakes\",\"price\": 2.52,\"other\":[true,null]}\t";

    assert_eq!(parse_price_response(body).unwrap(), Decimal::new(252, 2));
}

#[test]
fn requires_a_json_object_and_single_numeric_price_field() {
    for body in [
        "",
        "not JSON",
        "null",
        "2.52",
        "[2.52]",
        "{}",
        "{",
        r#"{"price":null}"#,
        r#"{"price":true}"#,
        r#"{"price":"2.52"}"#,
        r#"{"price":[]}"#,
        r#"{"price":{}}"#,
        r#"{"price":2.52,"price":9.98}"#,
        r#"{"price":2.52} trailing garbage"#,
        r#"{"price":2.52}{"price":9.98}"#,
        r#"{"price":NaN}"#,
        r#"{"price":Infinity}"#,
    ] {
        let error = parse_price_response(body).unwrap_err();

        assert!(
            matches!(&error, PriceResponseError::InvalidResponse(_)),
            "{body}: {error}"
        );
        assert!(error.source().is_some());
        assert!(error.to_string().contains("price response"));
    }
}

#[test]
fn rejects_objects_mimicking_serde_internal_tokens() {
    for body in [
        r#"{"price":{"$serde_json::private::Number":"2.52"}}"#,
        r#"{"price":{"$serde_json::private::RawValue":"2.52"}}"#,
    ] {
        assert!(matches!(
            parse_price_response(body),
            Err(PriceResponseError::InvalidResponse(_))
        ));
    }
}

#[test]
fn rejects_invalid_money_and_retains_its_exact_numeric_token() {
    for literal in [
        "-0.01",
        "1.001",
        "184467440737095516.16",
        "184467440737095517",
        "79228162514264337593543950336",
        "0.01000000000000000000000000001",
        "1.0010000000000000000000000000000000000000",
        "-0.0100000000000000000000000000000000000000",
        "184467440737095516.1600000000000000000000",
    ] {
        let body = format!(r#"{{"price":{literal}}}"#);
        let error = parse_price_response(&body).unwrap_err();

        assert!(
            matches!(&error, PriceResponseError::InvalidPrice(value) if value == literal),
            "{body}: {error}"
        );
        assert!(error.source().is_none());
        assert!(error.to_string().contains("exact cents"));
    }
}

#[test]
fn rejects_exponent_notation_even_when_it_denotes_exact_cents() {
    for literal in [
        "2.52e0",
        "252e-2",
        "1e-29",
        "1E2",
        "2.52000000000000000000000000000E0",
        "2.52000000000000000000000000000e+000",
    ] {
        let body = format!(r#"{{"price":{literal}}}"#);

        assert!(matches!(
            parse_price_response(&body),
            Err(PriceResponseError::InvalidPrice(value)) if value == literal
        ));
    }
}

#[test]
fn accepts_the_exact_body_limit_and_rejects_one_additional_byte() {
    let json = r#"{"price":2.52}"#;
    let mut body = format!(
        "{}{}",
        " ".repeat(MAX_PRICE_RESPONSE_BYTES - json.len()),
        json
    );

    assert_eq!(body.len(), MAX_PRICE_RESPONSE_BYTES);
    assert_eq!(parse_price_response(&body).unwrap(), Decimal::new(252, 2));

    body.push(' ');
    let error = parse_price_response(&body).unwrap_err();
    assert!(matches!(&error, PriceResponseError::InvalidResponse(_)));
    assert!(error.to_string().contains("16 KiB"));
}

#[test]
fn bounds_utf8_bytes_instead_of_character_count() {
    let body = format!(r#"{{"title":"{}","price":2.52}}"#, "é".repeat(8_190));

    assert!(body.chars().count() < MAX_PRICE_RESPONSE_BYTES);
    assert!(body.len() > MAX_PRICE_RESPONSE_BYTES);
    assert!(matches!(
        parse_price_response(&body),
        Err(PriceResponseError::InvalidResponse(_))
    ));
}
