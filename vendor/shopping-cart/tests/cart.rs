use std::{cell::RefCell, collections::VecDeque, error::Error, io};

use equal_experts_shopping_cart::{CartError, CartSnapshot, PriceProvider, ShoppingCart};
use rust_decimal::Decimal;

#[derive(Default)]
struct FakePrices {
    replies: RefCell<VecDeque<Result<Decimal, io::Error>>>,
    calls: RefCell<Vec<String>>,
}

impl FakePrices {
    fn returning(prices: &[&str]) -> Self {
        Self {
            replies: RefCell::new(prices.iter().map(|price| Ok(decimal(price))).collect()),
            ..Self::default()
        }
    }
}

impl PriceProvider for &FakePrices {
    type Error = io::Error;

    fn unit_price(&self, product: &str) -> Result<Decimal, Self::Error> {
        self.calls.borrow_mut().push(product.to_owned());
        self.replies
            .borrow_mut()
            .pop_front()
            .expect("the test must provide a response for each price lookup")
    }
}

fn decimal(value: &str) -> Decimal {
    Decimal::from_str_exact(value).unwrap()
}

fn assert_totals(state: &CartSnapshot, subtotal: &str, tax: &str, total: &str) {
    assert_eq!(state.subtotal, decimal(subtotal));
    assert_eq!(state.tax, decimal(tax));
    assert_eq!(state.total, decimal(total));
    assert_eq!(state.subtotal.scale(), 2);
    assert_eq!(state.tax.scale(), 2);
    assert_eq!(state.total.scale(), 2);
}

#[test]
fn empty_cart_has_zero_totals_and_does_not_request_prices() {
    let prices = FakePrices::default();
    let cart = ShoppingCart::new(&prices);

    let state = cart.snapshot();

    assert!(state.items.is_empty());
    assert_totals(&state, "0.00", "0.00", "0.00");
    assert!(prices.calls.borrow().is_empty());
}

#[test]
fn matches_the_assignment_example_including_repeated_product_quantities() {
    let prices = FakePrices::returning(&["2.52", "2.52", "9.98"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("cornflakes", 1).unwrap();
    cart.add("cornflakes", 1).unwrap();
    cart.add("weetabix", 1).unwrap();

    let state = cart.snapshot();
    assert_eq!(state.items.len(), 2);
    assert_eq!(state.items["cornflakes"].quantity, 2);
    assert_eq!(state.items["cornflakes"].unit_price, decimal("2.52"));
    assert_eq!(state.items["weetabix"].quantity, 1);
    assert_eq!(state.items["weetabix"].unit_price, decimal("9.98"));
    assert_totals(&state, "15.02", "1.88", "16.90");
    assert_eq!(
        *prices.calls.borrow(),
        ["cornflakes", "cornflakes", "weetabix"]
    );
}

#[test]
fn multiplies_quantities_and_sums_prices_without_binary_float_rounding() {
    let prices = FakePrices::returning(&["0.10", "0.20"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("cornflakes", 3).unwrap();
    cart.add("weetabix", 1).unwrap();

    assert_totals(&cart.snapshot(), "0.50", "0.07", "0.57");
}

#[test]
fn ceiling_tax_handles_below_half_cent_half_cent_and_exact_cent_boundaries() {
    for (subtotal, tax, total) in [
        ("0.01", "0.01", "0.02"),
        ("0.04", "0.01", "0.05"),
        ("0.08", "0.01", "0.09"),
        ("0.09", "0.02", "0.11"),
    ] {
        let prices = FakePrices::returning(&[subtotal]);
        let mut cart = ShoppingCart::new(&prices);

        cart.add("cornflakes", 1).unwrap();

        assert_totals(&cart.snapshot(), subtotal, tax, total);
    }
}

#[test]
fn rounds_tax_once_for_the_cart_instead_of_rounding_each_line() {
    let prices = FakePrices::returning(&["0.04", "0.04"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("cornflakes", 1).unwrap();
    cart.add("weetabix", 1).unwrap();

    assert_totals(&cart.snapshot(), "0.08", "0.01", "0.09");
}

#[test]
fn latest_fetched_price_applies_to_all_units_of_a_repeated_product() {
    let prices = FakePrices::returning(&["0.10", "0.20"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("cornflakes", 2).unwrap();
    cart.add("cornflakes", 1).unwrap();

    let state = cart.snapshot();
    assert_eq!(state.items["cornflakes"].quantity, 3);
    assert_eq!(state.items["cornflakes"].unit_price, decimal("0.20"));
    assert_totals(&state, "0.60", "0.08", "0.68");
    assert_eq!(*prices.calls.borrow(), ["cornflakes", "cornflakes"]);
}

#[test]
fn snapshots_are_detached_and_reads_do_not_retrieve_prices() {
    let prices = FakePrices::returning(&["2.52"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", 1).unwrap();
    let expected = cart.snapshot();

    let mut changed = cart.snapshot();
    changed.items.get_mut("cornflakes").unwrap().quantity = 0;
    changed.items.clear();
    changed.total = Decimal::ZERO;

    assert_eq!(cart.snapshot(), expected);
    assert_eq!(*prices.calls.borrow(), ["cornflakes"]);
}

#[test]
fn accepts_free_products_and_the_largest_quantity() {
    let prices = FakePrices::returning(&["0.00"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("cornflakes", u32::MAX).unwrap();

    let state = cart.snapshot();
    assert_eq!(state.items["cornflakes"].quantity, u32::MAX);
    assert_totals(&state, "0.00", "0.00", "0.00");
}

#[test]
fn accepts_lowercase_slug_identifiers_without_hardcoding_the_catalog() {
    let prices = FakePrices::returning(&["1.00"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("whole-grain2", 1).unwrap();

    assert_eq!(cart.snapshot().items["whole-grain2"].quantity, 1);
    assert_eq!(*prices.calls.borrow(), ["whole-grain2"]);
}

#[test]
fn accepts_harmless_trailing_zeros_and_normalizes_unit_prices_to_cents() {
    let prices = FakePrices::returning(&["2.5200"]);
    let mut cart = ShoppingCart::new(&prices);

    cart.add("cornflakes", 1).unwrap();

    let state = cart.snapshot();
    assert_eq!(state.items["cornflakes"].unit_price.to_string(), "2.52");
    assert_totals(&state, "2.52", "0.32", "2.84");
}

#[test]
fn zero_quantity_preserves_existing_state_without_a_lookup() {
    let prices = FakePrices::returning(&["2.52"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", 1).unwrap();
    let before = cart.snapshot();

    let error = cart.add("cornflakes", 0).unwrap_err();

    assert!(matches!(&error, CartError::InvalidQuantity));
    assert!(error.to_string().contains("positive integer"));
    assert_eq!(cart.snapshot(), before);
    assert_eq!(*prices.calls.borrow(), ["cornflakes"]);
}

#[test]
fn invalid_identifiers_are_rejected_before_lookup_without_changing_state() {
    let prices = FakePrices::returning(&["2.52"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", 1).unwrap();
    let before = cart.snapshot();

    for product in [
        "",
        "Cornflakes",
        "corn flakes",
        "../cornflakes",
        "cornflakes/weetabix",
        "cornflakes?quantity=2",
        "cornflakes#fragment",
        "cornflákes",
        "-cornflakes",
        "cornflakes-",
        "corn--flakes",
        "corn_flakes",
        "cornflakes\n",
    ] {
        let error = cart.add(product, 1).unwrap_err();

        assert!(matches!(&error, CartError::InvalidProduct(value) if value == product));
        assert!(error.to_string().contains("lowercase ASCII slug"));
        assert_eq!(cart.snapshot(), before);
    }
    assert_eq!(*prices.calls.borrow(), ["cornflakes"]);
}

#[test]
fn invalid_prices_cannot_reprice_or_increment_existing_items() {
    for invalid in [
        decimal("-0.01"),
        decimal("1.001"),
        decimal("184467440737095516.16"),
        decimal("184467440737095517"),
        Decimal::MAX,
    ] {
        let prices = FakePrices::returning(&["2.52"]);
        prices.replies.borrow_mut().push_back(Ok(invalid));
        let mut cart = ShoppingCart::new(&prices);
        cart.add("cornflakes", 1).unwrap();
        let before = cart.snapshot();

        let error = cart.add("cornflakes", 2).unwrap_err();

        assert!(matches!(&error, CartError::InvalidPrice(value) if *value == invalid));
        assert!(error.to_string().contains("invalid unit price"));
        assert_eq!(cart.snapshot(), before);
    }
}

#[test]
fn provider_failure_preserves_state_and_the_original_error() {
    for product in ["cornflakes", "weetabix"] {
        let prices = FakePrices::returning(&["2.52"]);
        prices
            .replies
            .borrow_mut()
            .push_back(Err(io::Error::other("price service unavailable")));
        let mut cart = ShoppingCart::new(&prices);
        cart.add("cornflakes", 1).unwrap();
        let before = cart.snapshot();

        let error = cart.add(product, 2).unwrap_err();

        assert!(matches!(&error, CartError::PriceLookup(_)));
        assert_eq!(
            error.source().unwrap().to_string(),
            "price service unavailable"
        );
        assert!(error.to_string().contains("price lookup failed"));
        assert_eq!(cart.snapshot(), before);
    }
}

#[test]
fn quantity_overflow_is_rejected_before_lookup() {
    let prices = FakePrices::returning(&["0.00"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", u32::MAX).unwrap();
    let before = cart.snapshot();

    let error = cart.add("cornflakes", 1).unwrap_err();

    assert!(matches!(&error, CartError::QuantityOverflow));
    assert!(error.to_string().contains("u32::MAX"));
    assert_eq!(cart.snapshot(), before);
    assert_eq!(*prices.calls.borrow(), ["cornflakes"]);
}

#[test]
fn line_amount_overflow_preserves_the_old_quantity_and_price() {
    let prices = FakePrices::returning(&["0.01", "184467440737095516.15"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", 2).unwrap();
    let before = cart.snapshot();

    let error = cart.add("cornflakes", 1).unwrap_err();

    assert!(matches!(&error, CartError::AmountOverflow));
    assert!(error.to_string().contains("supported maximum"));
    assert_eq!(cart.snapshot(), before);
}

#[test]
fn subtotal_overflow_does_not_add_a_new_product() {
    let prices = FakePrices::returning(&["100000000000000000.00", "100000000000000000.00"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", 1).unwrap();
    let before = cart.snapshot();

    let error = cart.add("weetabix", 1).unwrap_err();

    assert!(matches!(error, CartError::AmountOverflow));
    assert_eq!(cart.snapshot(), before);
}

#[test]
fn exact_maximum_total_is_supported_but_one_more_cent_is_atomic_failure() {
    let prices = FakePrices::returning(&["163971058432973792.13", "0.01"]);
    let mut cart = ShoppingCart::new(&prices);
    cart.add("cornflakes", 1).unwrap();
    let before = cart.snapshot();
    assert_totals(
        &before,
        "163971058432973792.13",
        "20496382304121724.02",
        "184467440737095516.15",
    );

    let error = cart.add("weetabix", 1).unwrap_err();

    assert!(matches!(error, CartError::AmountOverflow));
    assert_eq!(cart.snapshot(), before);
}
