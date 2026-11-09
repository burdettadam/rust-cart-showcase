import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import init, { BrowserCart } from '../dist/pkg/cart_wasm.js';

const binary = await readFile(new URL('../dist/pkg/cart_wasm_bg.wasm', import.meta.url));
await init({ module_or_path: binary });
const cart = new BrowserCart();
const checks = [];
function check(name, fn) { fn(); checks.push(name); }
const snapshot = () => JSON.parse(cart.snapshot());
const add = (product, quantity, raw) => JSON.parse(cart.add(product, quantity, raw));
try {
  check('empty WASM cart', () => assert.equal(snapshot().total, '0.00'));
  check('official example executes through compiled WASM', () => {
    assert.equal(add('cornflakes', '1', '{"price":2.52}').ok, true);
    assert.equal(add('cornflakes', '1', '{"price":2.52}').ok, true);
    assert.equal(add('weetabix', '1', '{"price":9.98}').ok, true);
    assert.deepEqual(snapshot(), { items: [
      { product: 'cornflakes', quantity: 2, unit_price: '2.52', line_total: '5.04' },
      { product: 'weetabix', quantity: 1, unit_price: '9.98', line_total: '9.98' }
    ], subtotal: '15.02', tax: '1.88', total: '16.90' });
  });
  check('failed inputs preserve compiled-WASM state', () => {
    const before = snapshot();
    for (const [quantity, body] of [
      ['-1', '{"price":2.52}'], ['1.5', '{"price":2.52}'], ['4294967296', '{"price":2.52}'],
      ['1', '{"price":2.525}'], ['1', '[2.52]'], ['1', '{"price":{"$serde_json::private::Number":"2.52"}}'],
      ['1', '{"price":2.52,"price":1.00}'], ['1', '\uFEFF{"price":2.52}'], ['1', ' '.repeat(16385)],
      ['1', '{"price":2.5200e0}'], ['1', `{"price":0.01${'0'.repeat(60)}1}`]
    ]) {
      const result = add('cornflakes', quantity, body);
      assert.equal(result.ok, false);
      assert.ok(result.error);
      assert.deepEqual(result.snapshot, before);
    }
  });
  check('latest-price repricing and upward tax', () => {
    cart.reset();
    add('cornflakes', '1', '{"price":2.52}');
    const result = add('cornflakes', '1', '{"price":3.25}');
    assert.equal(result.snapshot.subtotal, '6.50');
    assert.equal(result.snapshot.tax, '0.82');
    assert.equal(result.snapshot.total, '7.32');
  });
  check('one-cent tax ceiling', () => {
    cart.reset();
    const result = add('sample', '1', '{"price":0.01}');
    assert.equal(result.snapshot.tax, '0.01');
    assert.equal(result.snapshot.total, '0.02');
  });
  check('decimal precision across JS/WASM string boundary', () => {
    cart.reset();
    const result = add('sample', '1', '{"price":90071992547409.91}');
    assert.equal(result.snapshot.subtotal, '90071992547409.91');
    assert.equal(result.snapshot.total, '101330991615836.15');
  });
  check('insignificant trailing zeros preserve exact cents through compiled WASM', () => {
    for (const [price, total] of [['2.52', '2.84'], ['0.00', '0.00'], ['90071992547409.91', '101330991615836.15']]) {
      cart.reset();
      const result = add('sample', '1', `{"price":${price}${'0'.repeat(60)}}`);
      assert.equal(result.ok, true, result.error);
      assert.equal(result.snapshot.items[0].unit_price, price);
      assert.equal(result.snapshot.subtotal, price);
      assert.equal(result.snapshot.total, total);
    }
  });
  check('overflow is a returned error, not a WASM trap', () => {
    cart.reset();
    assert.equal(add('sample', '4294967295', '{"price":0}').ok, true);
    const before = snapshot();
    assert.equal(add('sample', '1', '{"price":0}').ok, false);
    assert.equal(add('expensive', '1', '{"price":184467440737095516.15}').ok, false);
    assert.deepEqual(snapshot(), before);
  });
  check('reset and independent WASM instances', () => {
    const other = new BrowserCart();
    try { assert.equal(JSON.parse(other.snapshot()).total, '0.00'); }
    finally { other.free(); }
    assert.deepEqual(JSON.parse(cart.reset()), { items: [], subtotal: '0.00', tax: '0.00', total: '0.00' });
  });
} finally { cart.free(); }
console.log(JSON.stringify({ runtime: process.version, target: 'wasm32-unknown-unknown', passed: checks.length, checks }, null, 2));
