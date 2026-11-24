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

#[test]
fn third_unit_is_free_in_addition_order_across_products() {
    for (steps, free_product, discount, subtotal, tax, total) in [
        (
            [("cornflakes", "2.52", 2), ("weetabix", "9.98", 1)],
            "weetabix",
            "9.98",
            "5.04",
            "0.63",
            "5.67",
        ),
        (
            [("weetabix", "9.98", 1), ("cornflakes", "2.52", 2)],
            "cornflakes",
            "2.52",
            "12.50",
            "1.57",
            "14.07",
        ),
    ] {
        let prices = FakePrices::returning(&[steps[0].1, steps[1].1]);
        let mut cart = ShoppingCart::with_every_third_free(&prices);
        for (product, _, quantity) in steps {
            cart.add(product, quantity).unwrap();
        }
        let receipt = cart.receipt();
        assert_eq!(receipt.free_quantities.len(), 1);
        assert_eq!(receipt.free_quantities[free_product], 1);
        assert_eq!(receipt.merchandise_subtotal, decimal("15.02"));
        assert_eq!(receipt.discount, decimal(discount));
        assert_totals(&receipt.snapshot, subtotal, tax, total);
    }
}

#[test]
fn bulk_add_matches_single_units_across_multiple_groups() {
    let prices = FakePrices::returning(&["1", "2"]);
    let mut bulk = ShoppingCart::with_every_third_free(&prices);
    bulk.add("a", 2).unwrap();
    bulk.add("b", 5).unwrap();

    let prices = FakePrices::returning(&["1", "1", "2", "2", "2", "2", "2"]);
    let mut singles = ShoppingCart::with_every_third_free(&prices);
    for product in ["a", "a", "b", "b", "b", "b", "b"] {
        singles.add(product, 1).unwrap();
    }
    assert_eq!(bulk.receipt(), singles.receipt());
    assert_eq!(bulk.receipt().free_quantities["b"], 2);
    assert_totals(&bulk.snapshot(), "8.00", "1.00", "9.00");
}

#[test]
fn failed_adds_preserve_receipt_and_next_free_position() {
    let prices = FakePrices::returning(&["0.04"]);
    prices.replies.borrow_mut().extend([
        Err(io::Error::other("unavailable")),
        Ok(decimal("184467440737095516.15")),
        Ok(decimal("1.00")),
    ]);
    let mut cart = ShoppingCart::with_every_third_free(&prices);
    cart.add("a", 2).unwrap();
    let before = cart.receipt();
    assert!(matches!(cart.add("b", 1), Err(CartError::PriceLookup(_))));
    assert_eq!(cart.receipt(), before);
    assert!(matches!(cart.add("a", 1), Err(CartError::AmountOverflow)));
    assert_eq!(cart.receipt(), before);
    cart.add("c", 1).unwrap();
    assert_eq!(cart.receipt().free_quantities["c"], 1);
    assert_totals(&cart.snapshot(), "0.08", "0.01", "0.09");
}

#[test]
fn discount_reprices_with_product_and_tax_rounds_once_after_discount() {
    let prices = FakePrices::returning(&["1", "2"]);
    let mut cart = ShoppingCart::with_every_third_free(&prices);
    cart.add("a", 3).unwrap();
    cart.add("a", 1).unwrap();
    assert_eq!(cart.receipt().free_quantities["a"], 1);
    assert_eq!(cart.receipt().discount, decimal("2.00"));
    assert_totals(&cart.snapshot(), "6.00", "0.75", "6.75");

    let prices = FakePrices::returning(&["0.04", "0.04", "1.00"]);
    let mut cart = ShoppingCart::with_every_third_free(&prices);
    for product in ["a", "b", "c"] {
        cart.add(product, 1).unwrap();
    }
    assert_totals(&cart.snapshot(), "0.08", "0.01", "0.09");
}

#[test]
fn zero_price_units_count_and_maximum_batch_handles_existing_offset() {
    let prices = FakePrices::returning(&["1", "0", "1"]);
    let mut cart = ShoppingCart::with_every_third_free(&prices);
    cart.add("a", 2).unwrap();
    cart.add("b", u32::MAX).unwrap();
    assert_eq!(cart.receipt().free_quantities["b"], 1_431_655_765);
    assert_eq!(cart.receipt().discount, decimal("0.00"));
    cart.add("a", 1).unwrap();
    assert_eq!(cart.receipt().free_quantities["a"], 1);
    assert_totals(&cart.snapshot(), "2.00", "0.25", "2.25");
}

#[test]
fn generated_additions_match_a_unit_by_unit_receipt_after_each_add() {
    // Each digit selects one of two products and a quantity from one through four.
    for sequence in 0..8_usize.pow(3) {
        let quotes = FakePrices::returning(&["0.01", "0.00", "2.52"]);
        let mut cart = ShoppingCart::with_every_third_free(&quotes);
        let mut units = Vec::new();
        let mut prices = [0_u32; 2];
        let mut choices = sequence;
        for cents in [1, 0, 252] {
            let choice = choices % 8;
            choices /= 8;
            let product = choice % 2;
            let quantity = 1 + choice / 2;
            prices[product] = cents;
            units.extend(std::iter::repeat_n(product, quantity));
            cart.add(["a", "b"][product], quantity as u32).unwrap();
            let mut merchandise = 0;
            let mut savings = 0;
            let mut counts = [0; 2];
            let mut free = [0; 2];
            for (position, &sku) in units.iter().enumerate() {
                counts[sku] += 1;
                merchandise += prices[sku];
                if (position + 1) % 3 == 0 {
                    free[sku] += 1;
                    savings += prices[sku];
                }
            }
            let receipt = cart.receipt();
            for (sku, name) in ["a", "b"].iter().enumerate() {
                assert_eq!(
                    receipt
                        .snapshot
                        .items
                        .get(*name)
                        .map_or(0, |item| item.quantity),
                    counts[sku]
                );
                assert_eq!(
                    receipt.free_quantities.get(*name).copied().unwrap_or(0),
                    free[sku]
                );
            }
            let subtotal = merchandise - savings;
            let tax = (subtotal * 125).div_ceil(1000);
            let money = |value| Decimal::new(i64::from(value), 2);
            assert_eq!(receipt.merchandise_subtotal, money(merchandise));
            assert_eq!(receipt.discount, money(savings));
            assert_eq!(receipt.snapshot.subtotal, money(subtotal));
            assert_eq!(receipt.snapshot.tax, money(tax));
            assert_eq!(receipt.snapshot.total, money(subtotal + tax));
        }
    }
}

#[test]
fn decimal_tax_preserves_cent_boundaries_at_small_and_large_amounts() {
    for base_cents in [0_u64, 16_000_000_000_000_000_000] {
        for extra_cents in 0..16 {
            let subtotal_cents = base_cents + extra_cents;
            let price = Decimal::from_i128_with_scale(i128::from(subtotal_cents), 2);
            let prices = FakePrices::returning(&[&price.to_string()]);
            let mut cart = ShoppingCart::new(&prices);
            cart.add("sample", 1).unwrap();

            let expected_tax_cents = subtotal_cents.div_ceil(8);
            let expected_tax = Decimal::from_i128_with_scale(i128::from(expected_tax_cents), 2);
            let expected_total = price + expected_tax;
            let state = cart.snapshot();
            assert_totals(
                &state,
                &price.to_string(),
                &expected_tax.to_string(),
                &expected_total.to_string(),
            );
        }
    }
}

#[test]
fn receipt_money_keeps_two_decimal_places_including_zero_values() {
    for (price, quantity) in [("0", u32::MAX), ("1", 1), ("1", 3)] {
        let prices = FakePrices::returning(&[price]);
        let mut cart = ShoppingCart::with_every_third_free(&prices);
        cart.add("sample", quantity).unwrap();
        let receipt = cart.receipt();
        for amount in [
            receipt.snapshot.items["sample"].unit_price,
            receipt.merchandise_subtotal,
            receipt.discount,
            receipt.snapshot.subtotal,
            receipt.snapshot.tax,
            receipt.snapshot.total,
        ] {
            assert_eq!(amount.scale(), 2, "{price} × {quantity}: {amount}");
        }
    }
}
