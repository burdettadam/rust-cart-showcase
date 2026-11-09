use cart_wasm::BrowserCart;
use serde_json::{Value, json};

fn snapshot(cart: &BrowserCart) -> Value {
    serde_json::from_str(&cart.snapshot()).unwrap()
}

fn add(cart: &mut BrowserCart, product: &str, quantity: &str, response: &str) -> Value {
    serde_json::from_str(&cart.add(product, quantity, response)).unwrap()
}

#[test]
fn empty_and_default_instances_have_exact_zero_totals() {
    let expected = json!({"items": [], "subtotal": "0.00", "tax": "0.00", "total": "0.00"});
    assert_eq!(snapshot(&BrowserCart::new()), expected);
    assert_eq!(snapshot(&BrowserCart::default()), expected);
}

#[test]
fn supplied_example_returns_exact_strings_and_combined_quantities() {
    let mut cart = BrowserCart::new();
    for (product, price) in [
        ("cornflakes", "2.52"),
        ("cornflakes", "2.52"),
        ("weetabix", "9.98"),
    ] {
        let result = add(&mut cart, product, "1", &format!(r#"{{"price":{price}}}"#));
        assert_eq!(result["ok"], true);
        assert!(result["error"].is_null());
    }
    assert_eq!(
        snapshot(&cart),
        json!({
            "items": [
                {"product": "cornflakes", "quantity": 2, "unit_price": "2.52", "line_total": "5.04"},
                {"product": "weetabix", "quantity": 1, "unit_price": "9.98", "line_total": "9.98"}
            ], "subtotal": "15.02", "tax": "1.88", "total": "16.90"
        })
    );
}

#[test]
fn quantity_strings_cannot_wrap_or_silently_truncate() {
    let mut cart = BrowserCart::new();
    add(&mut cart, "cornflakes", "1", r#"{"price":2.52}"#);
    let before = snapshot(&cart);
    for quantity in [
        "",
        "0",
        "-1",
        "+1",
        "1.5",
        "1e3",
        " 1",
        "1 ",
        "NaN",
        "4294967296",
        "１２",
    ] {
        let result = add(&mut cart, "cornflakes", quantity, r#"{"price":2.52}"#);
        assert_eq!(result["ok"], false, "{quantity}");
        assert!(result["error"].as_str().unwrap().contains("quantity"));
        assert_eq!(result["snapshot"], before);
        assert_eq!(snapshot(&cart), before);
    }
}

#[test]
fn latest_quote_reprices_existing_units_using_the_same_core_rule() {
    let mut cart = BrowserCart::new();
    add(&mut cart, "cornflakes", "1", r#"{"price":2.52}"#);
    let result = add(&mut cart, "cornflakes", "1", r#"{"price":3.25}"#);
    assert_eq!(result["snapshot"]["items"][0]["unit_price"], "3.25");
    assert_eq!(result["snapshot"]["subtotal"], "6.50");
    assert_eq!(result["snapshot"]["tax"], "0.82");
    assert_eq!(result["snapshot"]["total"], "7.32");
}

#[test]
fn upward_tax_rounding_is_visible_for_one_cent() {
    let mut cart = BrowserCart::new();
    let result = add(&mut cart, "sample", "1", r#"{"price":0.01}"#);
    assert_eq!(result["snapshot"]["subtotal"], "0.01");
    assert_eq!(result["snapshot"]["tax"], "0.01");
    assert_eq!(result["snapshot"]["total"], "0.02");
}

#[test]
fn large_prices_do_not_round_through_javascript_number_precision() {
    let mut cart = BrowserCart::new();
    let result = add(&mut cart, "sample", "1", r#"{"price":90071992547409.91}"#);
    assert_eq!(result["ok"], true);
    assert_eq!(result["snapshot"]["subtotal"], "90071992547409.91");
    assert_eq!(result["snapshot"]["tax"], "11258999068426.24");
    assert_eq!(result["snapshot"]["total"], "101330991615836.15");
}

#[test]
fn malformed_and_invalid_prices_leave_the_cart_unchanged() {
    let mut cart = BrowserCart::new();
    add(&mut cart, "cornflakes", "1", r#"{"price":2.52}"#);
    let before = snapshot(&cart);
    for response in [
        "not-json",
        "\u{feff}{\"price\":2.52}",
        "[2.52]",
        "{}",
        r#"{"price":"2.52"}"#,
        r#"{"price":{"$serde_json::private::Number":"2.52"}}"#,
        r#"{"price":2.525}"#,
        r#"{"price":-1}"#,
        r#"{"price":1e2}"#,
        r#"{"price":2.52,"price":1.00}"#,
    ] {
        let result = add(&mut cart, "cornflakes", "1", response);
        assert_eq!(result["ok"], false, "{response}");
        assert!(!result["error"].as_str().unwrap().is_empty());
        assert_eq!(result["snapshot"], before);
    }
    let result = add(&mut cart, "cornflakes", "1", &" ".repeat(16 * 1024 + 1));
    assert_eq!(result["ok"], false);
    assert_eq!(result["snapshot"], before);
}

#[test]
fn invalid_products_and_overflows_preserve_the_snapshot() {
    let mut cart = BrowserCart::new();
    add(&mut cart, "sample", "4294967295", r#"{"price":0}"#);
    let before = snapshot(&cart);
    for (product, quantity, body) in [
        ("sample", "1", r#"{"price":0}"#),
        ("../../other", "1", r#"{"price":1}"#),
        ("Cornflakes", "1", r#"{"price":1}"#),
        ("expensive", "1", r#"{"price":184467440737095516.15}"#),
    ] {
        let result = add(&mut cart, product, quantity, body);
        assert_eq!(result["ok"], false);
        assert_eq!(result["snapshot"], before);
    }
}

#[test]
fn reset_and_separate_instances_do_not_share_state() {
    let mut cart = BrowserCart::new();
    let other = BrowserCart::new();
    add(&mut cart, "cornflakes", "2", r#"{"price":2.52}"#);
    assert_eq!(snapshot(&other)["total"], "0.00");
    let cleared: Value = serde_json::from_str(&cart.reset()).unwrap();
    assert_eq!(cleared, snapshot(&other));
    assert_eq!(cleared, snapshot(&cart));
    let result = add(&mut cart, "weetabix", "1", r#"{"price":9.9800}"#);
    assert_eq!(result["snapshot"]["total"], "11.23");
}
