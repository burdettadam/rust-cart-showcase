import { CartSession, PRODUCTS } from "./session.js";

const byId = (id) => document.getElementById(id);
const elements = {
  products: byId("product-list"),
  cartLines: byId("cart-lines"),
  empty: byId("empty-cart"),
  lineCount: byId("line-count"),
  subtotal: byId("subtotal"),
  tax: byId("tax"),
  total: byId("total"),
  mobileTotal: byId("mobile-total"),
  snapshot: byId("snapshot-json"),
  status: byId("operation-status"),
  message: byId("operation-message"),
  messageIcon: byId("message-icon"),
  engine: byId("engine-status"),
  engineLabel: byId("engine-label"),
  demo: byId("playground"),
  sourceDescription: byId("source-description"),
  customForm: byId("custom-form"),
  customNote: byId("custom-mode-note"),
};
const productsById = new Map(PRODUCTS.map((product) => [product.id, product]));
let session;
let ready = false;

function node(tag, className, text) {
  const element = document.createElement(tag);
  if (className) element.className = className;
  if (text !== undefined) element.textContent = text;
  return element;
}

function productMark(product, extraClass = "") {
  const mark = node("span", `product-mark ${product.color} ${extraClass}`, product.letter);
  mark.setAttribute("aria-hidden", "true");
  return mark;
}

function createCatalog() {
  const fragment = document.createDocumentFragment();
  for (const product of PRODUCTS) {
    const form = node("form", "product-card");
    form.setAttribute("aria-label", `Add ${product.name}`);
    const info = node("div", "product-info");
    info.append(node("h4", "product-name", product.name), node("p", "product-note", product.note));
    const price = node("div", "product-price");
    price.dataset.priceFor = product.id;
    const quantityLabel = node("label", "sr-only", `Quantity for ${product.name}`);
    quantityLabel.htmlFor = `quantity-${product.id}`;
    const quantity = node("input", "quantity-field");
    quantity.id = quantityLabel.htmlFor;
    quantity.type = "text";
    quantity.inputMode = "numeric";
    quantity.autocomplete = "off";
    quantity.maxLength = 20;
    quantity.value = "1";
    quantity.dataset.engine = "";
    quantity.disabled = true;
    const add = node("button", "add-button", "Add");
    add.type = "submit";
    add.setAttribute("aria-label", `Add ${product.name} to cart`);
    add.dataset.engine = "";
    add.disabled = true;
    const arrow = node("span", "", "+");
    arrow.setAttribute("aria-hidden", "true");
    add.append(arrow);
    form.append(productMark(product), info, price, quantityLabel, quantity, add);
    form.addEventListener("submit", async (event) => {
      event.preventDefault();
      if (ready) await session.add(product.id, quantity.value);
    });
    fragment.append(form);

    const option = node("option", "", product.name);
    option.value = product.id;
    byId("custom-product").append(option);
  }
  elements.products.append(fragment);
  byId("custom-product").value = "cornflakes";
  renderPrices("sample");
}

function renderPrices(mode) {
  for (const price of document.querySelectorAll("[data-price-for]")) {
    const product = productsById.get(price.dataset.priceFor);
    price.classList.toggle("is-live", mode === "live");
    price.replaceChildren(document.createTextNode(mode === "live" ? "On add" : product.price), node("span", "", mode === "live" ? "live quote" : "per unit"));
  }
}

function render(state) {
  const { snapshot, mode, busy, error, message } = state;
  elements.status.classList.toggle("is-error", Boolean(error));
  elements.status.classList.toggle("is-busy", busy);
  elements.status.setAttribute("role", error ? "alert" : "status");
  elements.status.setAttribute("aria-live", error ? "assertive" : "polite");
  elements.message.textContent = message;
  elements.messageIcon.textContent = error ? "!" : busy ? "↻" : "↳";
  elements.demo.setAttribute("aria-busy", String(busy));
  elements.sourceDescription.textContent = mode === "live"
    ? "Fresh API prices on every add. Network access required."
    : "Repeatable sample prices. No price API request.";
  elements.customNote.textContent = mode === "live"
    ? "Select Sample prices above to try a custom response. Switching clears the live cart."
    : "Available with Sample prices. This quote overrides the selected product's sample price for this addition.";
  for (const input of document.querySelectorAll("input[name='price-mode']")) {
    input.checked = input.value === mode;
  }
  renderPrices(mode);

  const fragment = document.createDocumentFragment();
  for (const item of snapshot.items) {
    const product = productsById.get(item.product) || { name: item.product, letter: item.product.slice(0, 1).toUpperCase(), color: "sage" };
    const row = node("li", "cart-line");
    const info = node("div", "cart-line-info");
    info.append(node("strong", "", product.name), node("span", "", `${item.quantity} × ${item.unit_price}`));
    row.append(productMark(product, "cart-line-mark"), info, node("span", "line-total", item.line_total));
    fragment.append(row);
  }
  elements.cartLines.replaceChildren(fragment);
  elements.empty.hidden = snapshot.items.length > 0;
  elements.lineCount.textContent = `${snapshot.items.length} ${snapshot.items.length === 1 ? "line" : "lines"}`;
  elements.subtotal.textContent = snapshot.subtotal;
  elements.tax.textContent = snapshot.tax;
  elements.total.textContent = snapshot.total;
  elements.mobileTotal.textContent = snapshot.total;
  elements.snapshot.textContent = JSON.stringify(snapshot, null, 2);

  for (const control of document.querySelectorAll("[data-engine]")) control.disabled = !ready;
  for (const control of elements.products.querySelectorAll("button, input")) control.disabled = !ready || busy;
  for (const control of elements.customForm.querySelectorAll("button, input, select, textarea")) {
    control.disabled = !ready || busy || mode === "live";
  }
}

function showLoadFailure(error) {
  ready = false;
  elements.engine.classList.remove("is-loading");
  elements.engine.classList.add("is-error");
  elements.engineLabel.textContent = "Rust unavailable";
  elements.status.classList.add("is-error");
  elements.status.setAttribute("role", "alert");
  elements.status.setAttribute("aria-live", "assertive");
  elements.messageIcon.textContent = "!";
  elements.message.textContent = "The WebAssembly module could not load. Reload the page to try again. A downloaded copy must be served over HTTP, rather than opened as a local file.";
  elements.demo.setAttribute("aria-busy", "false");
  byId("reload-button").hidden = false;
  for (const control of document.querySelectorAll("[data-engine]")) control.disabled = true;
  console.error("The Rust WebAssembly module could not load:", error);
}

async function loadBuildLabel() {
  try {
    const response = await fetch("./build-info.json");
    if (!response.ok) return;
    const metadata = await response.json();
    if (!/^[a-f0-9]{40}$/.test(metadata.core_commit) || !/^[a-f0-9]{40}$/.test(metadata.source_commit)) return;
    const label = byId("build-label");
    label.textContent = `Core ${metadata.core_commit.slice(0, 7)} · Showcase ${metadata.source_commit.slice(0, 7)}${metadata.source_dirty ? " + local changes" : ""}`;
    label.title = `Core: ${metadata.core_commit}\nShowcase: ${metadata.source_commit}${metadata.source_dirty ? " (built with local changes)" : ""}`;
  } catch {
    // Build metadata is optional; it never gates cart functionality.
  }
}

createCatalog();
for (const radio of document.querySelectorAll("input[name='price-mode']")) {
  radio.addEventListener("change", () => {
    if (ready && radio.checked) session.setMode(radio.value);
  });
}
byId("reset-cart").addEventListener("click", () => {
  if (ready) session.reset();
});
for (const button of document.querySelectorAll("[data-scenario]")) {
  button.addEventListener("click", () => {
    if (!ready) return;
    session.runScenario(button.dataset.scenario);
    elements.demo.scrollIntoView({ block: "start" });
  });
}
elements.customForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (ready) await session.add(byId("custom-product").value, byId("custom-quantity").value, byId("custom-body").value);
});
byId("reload-button").addEventListener("click", () => window.location.reload());

try {
  const { default: init, BrowserCart } = await import("./pkg/cart_wasm.js");
  await init();
  session = new CartSession(new BrowserCart(), { onChange: render });
  ready = true;
  elements.engine.classList.remove("is-loading");
  elements.engineLabel.textContent = "Rust + WASM ready";
  render(session.state);
} catch (error) {
  showLoadFailure(error);
}
void loadBuildLabel();
