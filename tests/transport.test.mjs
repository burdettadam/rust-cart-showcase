import assert from "node:assert/strict";
import test from "node:test";
import { fetchPriceBody, MAX_RESPONSE_BYTES, PRICE_API } from "../web/transport.js";

const code = (expected) => (error) => error.code === expected;
const raw = "{\"price\":184467440737095516.15}";

test("passes exact response text and uses a credential-free, uncached JSON request", async () => {
  let request;
  const actual = await fetchPriceBody("cornflakes", {
    fetchImpl: async (...args) => {
      request = args;
      return new Response(raw);
    },
  });
  assert.equal(actual, raw);
  assert.equal(request[0], `${PRICE_API}/cornflakes.json`);
  assert.equal(request[1].credentials, "omit");
  assert.equal(request[1].cache, "no-store");
  assert.equal(request[1].redirect, "error");
  assert.equal(request[1].headers.Accept, "application/json");
});

test("rejects HTTP failures before a quote reaches Rust", async () => {
  for (const status of [304, 404, 500]) {
    await assert.rejects(fetchPriceBody("cornflakes", {
      fetchImpl: async () => new Response(status === 304 ? null : raw, { status }),
    }), (error) => error.code === "status" && error.message.includes(String(status)));
  }
});

test("enforces the actual byte limit even without Content-Length", async () => {
  await assert.rejects(fetchPriceBody("cornflakes", {
    fetchImpl: async () => new Response("x".repeat(MAX_RESPONSE_BYTES + 1)),
  }), code("size"));
  assert.equal((await fetchPriceBody("cornflakes", {
    fetchImpl: async () => new Response("x".repeat(MAX_RESPONSE_BYTES)),
  })).length, MAX_RESPONSE_BYTES);
});

test("counts UTF-8 bytes, not characters", async () => {
  await assert.rejects(fetchPriceBody("cornflakes", {
    maxBytes: 3,
    fetchImpl: async () => new Response("éé"),
  }), code("size"));
});

test("rejects a declared oversized response before reading its body", async () => {
  await assert.rejects(fetchPriceBody("cornflakes", {
    fetchImpl: async () => new Response(raw, { headers: { "Content-Length": String(MAX_RESPONSE_BYTES + 1) } }),
  }), code("size"));
});

test("decodes a UTF-8 character spanning chunks and retains exact price digits", async () => {
  const bytes = new TextEncoder().encode('{"name":"café","price":2.5200}');
  const split = bytes.indexOf(0xc3) + 1;
  const stream = new ReadableStream({
    start(controller) {
      controller.enqueue(bytes.slice(0, split));
      controller.enqueue(bytes.slice(split));
      controller.close();
    },
  });
  const body = await fetchPriceBody("cornflakes", { fetchImpl: async () => new Response(stream) });
  assert.equal(body, '{"name":"café","price":2.5200}');
});

test("rejects invalid UTF-8 instead of silently replacing bytes", async () => {
  await assert.rejects(fetchPriceBody("cornflakes", {
    fetchImpl: async () => new Response(new Uint8Array([0xff])),
  }), code("network"));
});

test("times out a fetch even when its implementation ignores cancellation", async () => {
  await assert.rejects(fetchPriceBody("cornflakes", {
    timeoutMs: 5,
    fetchImpl: () => new Promise(() => {}),
  }), code("timeout"));
});

test("timeout includes the response body and cancels a stalled stream", async () => {
  let cancelled = false;
  const stream = new ReadableStream({ cancel() { cancelled = true; } });
  await assert.rejects(fetchPriceBody("cornflakes", {
    timeoutMs: 5,
    fetchImpl: async () => new Response(stream),
  }), code("timeout"));
  await Promise.resolve();
  assert.equal(cancelled, true);
});

test("external cancellation interrupts a pending fetch", async () => {
  const controller = new AbortController();
  let signal;
  const pending = fetchPriceBody("cornflakes", {
    signal: controller.signal,
    fetchImpl: async (_, options) => {
      signal = options.signal;
      return new Promise(() => {});
    },
  });
  controller.abort();
  await assert.rejects(pending, code("cancelled"));
  assert.equal(signal.aborted, true);
});

test("pre-cancelled and invalid requests never contact the API", async () => {
  let calls = 0;
  const fetchImpl = async () => { calls += 1; return new Response(raw); };
  await assert.rejects(fetchPriceBody("cornflakes", {
    signal: AbortSignal.abort(), fetchImpl,
  }), code("cancelled"));
  for (const product of ["../cornflakes", "x?secret=1", "Cornflakes", "a--b", ""]) {
    await assert.rejects(fetchPriceBody(product, { fetchImpl }), code("product"));
  }
  assert.equal(calls, 0);
});
