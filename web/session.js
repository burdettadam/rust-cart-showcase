import { fetchPriceBody } from "./transport.js";

export const PRODUCTS = Object.freeze([
  { id: "cheerios", name: "Cheerios", price: "8.43", letter: "C", color: "ochre", note: "A golden start" },
  { id: "cornflakes", name: "Cornflakes", price: "2.52", letter: "C", color: "sage", note: "The everyday classic" },
  { id: "frosties", name: "Frosties", price: "4.99", letter: "F", color: "blue", note: "A little sweetness" },
  { id: "shreddies", name: "Shreddies", price: "4.68", letter: "S", color: "rose", note: "Perfectly squared" },
  { id: "weetabix", name: "Weetabix", price: "9.98", letter: "W", color: "sand", note: "Keep it simple" },
]);

const fixtureBodies = new Map(PRODUCTS.map(({ id, price }) => [id, `{"price":${price}}`]));
const scenarios = {
  example: {
    message: "Reference example loaded: two cornflakes and one weetabix. Inspect the totals calculated by Rust.",
    steps: [["cornflakes", "1", "{\"price\":2.52}"], ["cornflakes", "1", "{\"price\":2.52}"], ["weetabix", "1", "{\"price\":9.98}"]],
  },
  rounding: {
    message: "Rounding scenario loaded. A 0.01 subtotal still produces a full cent of tax.",
    steps: [["cheerios", "1", "{\"price\":0.01}"]],
  },
  repricing: {
    message: "Repricing scenario loaded: two cornflakes at 2.52, then one at 3.00. The newest price applies to all three.",
    steps: [["cornflakes", "2", "{\"price\":2.52}"], ["cornflakes", "1", "{\"price\":3.00}"]],
  },
  rejection: {
    message: "",
    steps: [["cornflakes", "1", "{\"price\":2.52}"], ["frosties", "1", "{\"price\":2.525}"]],
  },
};

export class CartSession {
  constructor(cart, { loadPrice = fetchPriceBody, onChange = () => {} } = {}) {
    this.cart = cart;
    this.loadPrice = loadPrice;
    this.onChange = onChange;
    this.generation = 0;
    this.request = null;
    this.state = {
      mode: "sample",
      snapshot: JSON.parse(cart.snapshot()),
      busy: false,
      error: null,
      message: "Ready when you are. Add a product or run the reference example.",
      lastBody: null,
    };
  }

  emit() {
    this.onChange(this.state);
  }

  cancelPending() {
    this.generation += 1;
    this.request?.abort();
    this.request = null;
    this.state.busy = false;
  }

  reset() {
    this.cancelPending();
    this.state.snapshot = JSON.parse(this.cart.reset());
    this.state.error = null;
    this.state.lastBody = null;
    this.state.message = "Cart reset. Start a new example whenever you like.";
    this.emit();
  }

  setMode(mode) {
    if (mode !== "sample" && mode !== "live") throw new Error("Unknown price mode.");
    if (mode === this.state.mode) return;
    this.cancelPending();
    this.state.mode = mode;
    this.state.snapshot = JSON.parse(this.cart.reset());
    this.state.error = null;
    this.state.lastBody = null;
    this.state.message = mode === "live"
      ? "Live prices selected. The cart was cleared; each addition will request a fresh price."
      : "Sample prices selected. The cart was cleared; additions now use repeatable local prices.";
    this.emit();
  }

  apply(product, quantity, body) {
    const result = JSON.parse(this.cart.add(product, quantity, body));
    this.state.snapshot = result.snapshot;
    this.state.lastBody = body;
    this.state.error = result.ok ? null : result.error;
    this.state.message = result.ok
      ? `Added ${quantity} × ${product}. Rust updated the cart.`
      : `${result.error} Your cart is unchanged.`;
    return result.ok;
  }

  async add(product, quantity, customBody) {
    if (this.state.busy) return false;
    if (customBody !== undefined && this.state.mode !== "sample") {
      this.state.error = "custom-mode";
      this.state.message = "Select Sample prices before trying a custom response. Your cart is unchanged.";
      this.emit();
      return false;
    }
    const generation = this.generation;
    const controller = new AbortController();
    this.request = controller;
    this.state.busy = true;
    this.state.error = null;
    this.state.message = this.state.mode === "live"
      ? `Requesting a live price for ${product}…`
      : "Passing the quote to Rust…";
    this.emit();

    try {
      let body;
      if (customBody !== undefined) body = customBody;
      else if (this.state.mode === "sample") {
        body = fixtureBodies.get(product);
        if (body === undefined) throw new Error("No sample price is available for that product.");
      } else body = await this.loadPrice(product, { signal: controller.signal });

      if (generation !== this.generation) return false;
      return this.apply(product, quantity, body);
    } catch (error) {
      if (generation !== this.generation) return false;
      this.state.error = "request";
      this.state.message = `${error.message || "The operation could not be completed."} Your cart is unchanged.`;
      return false;
    } finally {
      if (generation === this.generation) {
        this.request = null;
        this.state.busy = false;
        this.emit();
      }
    }
  }

  runScenario(name) {
    const scenario = scenarios[name];
    if (!scenario) throw new Error("Unknown scenario.");
    this.cancelPending();
    this.state.mode = "sample";
    this.state.snapshot = JSON.parse(this.cart.reset());
    this.state.lastBody = null;
    this.state.error = null;
    let accepted = true;
    for (const step of scenario.steps) {
      if (!this.apply(...step)) {
        accepted = false;
        break;
      }
    }
    if (accepted) this.state.message = scenario.message;
    this.emit();
    return accepted;
  }
}
