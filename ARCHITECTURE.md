# Architecture authority

The companion calls the original Rust shopping-cart library through a small
WASM binding. Rust owns validation, quantity, pricing, state and totals. JavaScript
owns the DOM and asynchronous browser HTTP. Raw response text crosses the boundary;
decimal amounts return as strings. No JavaScript floating-point money arithmetic.

`vendor/shopping-cart` is an exported, pinned source snapshot. The original library
remains the only maintained rule implementation; update the snapshot using the
export tool and verify its manifest instead of hand-editing it. The showcase
ships independently of the library-only challenge submission.

Detailed design, tradeoffs and interview Q&A will accompany the implementation.
