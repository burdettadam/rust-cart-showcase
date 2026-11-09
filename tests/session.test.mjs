import assert from "node:assert/strict";
import test from "node:test";
import { CartSession } from "../web/session.js";

const empty = () => ({ items: [], subtotal: "0.00", tax: "0.00", total: "0.00" });
const deferred = () => {
  let resolve;
  let reject;
  const promise = new Promise((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
};

// This double verifies orchestration only. Actual cart math is tested in Rust/WASM.
class RecordingCart {
  constructor() { this.calls = []; this.value = empty(); this.nextError = null; }
  snapshot() { return JSON.stringify(this.value); }
  reset() { this.value = empty(); return this.snapshot(); }
  add(product, quantity, body) {
    this.calls.push({ product, quantity, body });
    if (this.nextError) {
      return JSON.stringify({ ok: false, error: this.nextError, snapshot: this.value });
    }
    this.value = { ...empty(), items: [{ product, quantity: 1, unit_price: "1.00", line_total: "1.00" }] };
    return JSON.stringify({ ok: true, error: null, snapshot: this.value });
  }
}

test("sample mode passes untouched quantity and exact numeric JSON text to Rust", async () => {
  const cart = new RecordingCart();
  const session = new CartSession(cart);
  await session.add("cornflakes", "4294967296");
  assert.deepEqual(cart.calls, [{ product: "cornflakes", quantity: "4294967296", body: '{"price":2.52}' }]);
  assert.equal(session.state.busy, false);
});

test("a live fetch is serialized and passes raw text without JSON number conversion", async () => {
  const quote = deferred();
  const cart = new RecordingCart();
  const session = new CartSession(cart, { loadPrice: () => quote.promise });
  session.setMode("live");
  const first = session.add("cornflakes", "2");
  assert.equal(await session.add("weetabix", "1"), false);
  assert.equal(cart.calls.length, 0);
  const exact = '{"price":184467440737095516.15}';
  quote.resolve(exact);
  assert.equal(await first, true);
  assert.deepEqual(cart.calls, [{ product: "cornflakes", quantity: "2", body: exact }]);
});

test("reset cancels a pending request and a late quote cannot repopulate the cart", async () => {
  const quote = deferred();
  let signal;
  const cart = new RecordingCart();
  const session = new CartSession(cart, {
    loadPrice: (_, options) => { signal = options.signal; return quote.promise; },
  });
  session.setMode("live");
  const pending = session.add("cornflakes", "1");
  session.reset();
  assert.equal(signal.aborted, true);
  quote.resolve('{"price":2.52}');
  assert.equal(await pending, false);
  assert.equal(cart.calls.length, 0);
  assert.deepEqual(session.state.snapshot, empty());
  assert.match(session.state.message, /Cart reset/);
});

test("a stale request cannot clear the busy state or overwrite a newer request", async () => {
  const a = deferred();
  const b = deferred();
  let count = 0;
  const cart = new RecordingCart();
  const session = new CartSession(cart, { loadPrice: () => (++count === 1 ? a.promise : b.promise) });
  session.setMode("live");
  const first = session.add("cornflakes", "1");
  session.reset();
  const second = session.add("weetabix", "3");
  a.reject(new Error("stale failure"));
  assert.equal(await first, false);
  assert.equal(session.state.busy, true);
  assert.match(session.state.message, /weetabix/);
  b.resolve('{"price":9.98}');
  await second;
  assert.equal(session.state.busy, false);
  assert.deepEqual(cart.calls.map((call) => call.product), ["weetabix"]);
});

test("mode changes clear the cart and invalidate old live responses", async () => {
  const quote = deferred();
  const cart = new RecordingCart();
  const session = new CartSession(cart, { loadPrice: () => quote.promise });
  session.setMode("live");
  const old = session.add("cornflakes", "1");
  session.setMode("sample");
  await session.add("weetabix", "1");
  const expected = structuredClone(session.state.snapshot);
  quote.resolve('{"price":2.52}');
  assert.equal(await old, false);
  assert.deepEqual(session.state.snapshot, expected);
  assert.equal(session.state.mode, "sample");
  assert.deepEqual(cart.calls.map((call) => call.product), ["weetabix"]);
});

test("transport failure preserves the existing cart and exposes a useful error", async () => {
  const cart = new RecordingCart();
  let fail = false;
  const session = new CartSession(cart, { loadPrice: async () => {
    if (fail) throw new Error("The price service returned HTTP 500.");
    return '{"price":2.52}';
  } });
  session.setMode("live");
  await session.add("cornflakes", "1");
  const previous = structuredClone(session.state.snapshot);
  fail = true;
  assert.equal(await session.add("frosties", "1"), false);
  assert.deepEqual(session.state.snapshot, previous);
  assert.equal(cart.calls.length, 1);
  assert.match(session.state.message, /HTTP 500.*unchanged/);
});

test("Rust rejection is rendered with Rust's unchanged snapshot", async () => {
  const cart = new RecordingCart();
  const session = new CartSession(cart);
  await session.add("cornflakes", "1");
  const previous = structuredClone(session.state.snapshot);
  cart.nextError = "quantity must be a positive integer";
  assert.equal(await session.add("cornflakes", "-1"), false);
  assert.deepEqual(session.state.snapshot, previous);
  assert.equal(session.state.error, cart.nextError);
  assert.match(session.state.message, /positive integer.*unchanged/);
});

test("an unexpected engine exception does not falsely promise an unchanged cart", async () => {
  const cart = new RecordingCart();
  const session = new CartSession(cart);
  cart.add = () => { throw new Error("unexpected WASM failure"); };
  assert.equal(await session.add("cornflakes", "1"), false);
  assert.equal(session.state.error, "engine");
  assert.match(session.state.message, /Reset the cart/);
  assert.doesNotMatch(session.state.message, /unchanged/);
  assert.equal(await session.add("cornflakes", "1"), false);
  session.reset();
  assert.equal(session.state.error, null);
});

test("custom responses stay verbatim and cannot be mixed into live mode", async () => {
  const cart = new RecordingCart();
  const session = new CartSession(cart);
  const body = '{"price":{"$serde_json::private::Number":"2.52"}}';
  await session.add("cornflakes", "1", body);
  assert.equal(cart.calls[0].body, body);
  session.setMode("live");
  assert.equal(await session.add("cornflakes", "1", body), false);
  assert.equal(cart.calls.length, 1);
  assert.match(session.state.message, /Select Sample prices/);
});

test("reference scenario resets, switches to sample mode and calls Rust for each addition", async () => {
  const cart = new RecordingCart();
  const quote = deferred();
  const session = new CartSession(cart, { loadPrice: () => quote.promise });
  session.setMode("live");
  const pending = session.add("shreddies", "1");
  assert.equal(session.runScenario("example"), true);
  assert.equal(session.state.mode, "sample");
  assert.deepEqual(cart.calls.map(({ product, quantity }) => [product, quantity]), [["cornflakes", "1"], ["cornflakes", "1"], ["weetabix", "1"]]);
  quote.resolve('{"price":4.68}');
  await pending;
  assert.equal(cart.calls.length, 3);
});
