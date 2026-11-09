# Browser interface and transport

The UI is a separate companion to the library-only challenge. The root agent
provides `BrowserCart` from `./pkg/cart_wasm.js`; JavaScript sends quantity and
price text unchanged, then displays the snapshot returned by Rust. The interface
does not calculate prices, subtotal, tax, line totals or grand total.

## Visible behavior

- Sample prices are repeatable local numeric JSON quotes. Live mode obtains a
  fresh quote on each addition from the public price API. Changing mode clears
  the cart so one basket cannot silently mix the two sources.
- The reference, rounding, repricing and rejection buttons start a fresh sample
  cart. Scenario labels distinguish expected behavior from the active cart.
- A custom raw JSON form is available only in sample mode. It demonstrates exact
  parsing and rejection without disguising a custom quote as a live API result.
- Snapshot inspection displays the actual JSON snapshot returned by WASM, with
  money as decimal strings. Source identities are shown from optional
  `build-info.json`; missing metadata never prevents the cart from loading.
- Native radio buttons, labels, visible focus, a skip link, status/error
  announcements, reduced-motion support and responsive cart navigation are
  included. Dynamic data uses DOM `textContent`, never HTML interpolation.

## Browser transport

`fetchPriceBody` omits credentials, uses no-store requests, rejects redirects,
checks HTTP success, decodes UTF-8 strictly and limits the streamed response to
16 KiB of bytes. The five-second budget covers both headers and body. An abort
race also settles a request when an injected fetch implementation ignores its
signal. Actual requests use the browser Fetch API.

`CartSession` permits one pending addition. Reset, mode changes and sample
scenarios cancel and invalidate a pending request. A generation check before
the WASM call prevents late data from mutating a new cart, and another check in
cleanup prevents an old request from clearing a newer request's busy state.

## Evidence and limits

The browser-controller checkpoints passed 21 Node tests with Node 24.22.1.
They exercise exact raw transport, HTTP failures, response byte boundaries,
UTF-8 decoding, full-request timeout, external cancellation, quantity forwarding,
serialized calls and reset/mode-change races. The controller double records
calls and simulates envelopes; it deliberately does not reimplement money math.
Actual Rust and compiled-WASM behavior is verified by the integration owner.
Unexpected engine exceptions block further additions and request a reset without
claiming the cart is unchanged; normal Rust rejections use the returned snapshot.

Syntax checks were run for all three JavaScript modules. Visual/browser
verification, fixed-candidate integration checks and independent reviews are
the integration owner's next steps. No hosted CI, public deployment or human
approval is claimed by these local checks.
