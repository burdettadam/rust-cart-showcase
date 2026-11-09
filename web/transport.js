export const PRICE_API = "https://equalexperts.github.io/backend-take-home-test-data";
export const MAX_RESPONSE_BYTES = 16 * 1024;
export const REQUEST_TIMEOUT_MS = 5000;

export class PriceRequestError extends Error {
  constructor(code, message) {
    super(message);
    this.name = "PriceRequestError";
    this.code = code;
  }
}

const cancelled = () => new PriceRequestError("cancelled", "The price request was cancelled.");

export async function fetchPriceBody(product, {
  signal,
  fetchImpl = globalThis.fetch,
  timeoutMs = REQUEST_TIMEOUT_MS,
  maxBytes = MAX_RESPONSE_BYTES,
} = {}) {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(product)) {
    throw new PriceRequestError("product", "The product cannot be used in a price request.");
  }
  if (signal?.aborted) throw cancelled();

  const controller = new AbortController();
  let reader;
  let timedOut = false;
  let abortListener;
  const abort = () => controller.abort();
  const interrupted = new Promise((_, reject) => {
    abortListener = () => reject(timedOut
      ? new PriceRequestError("timeout", "The price service took too long. Please try again.")
      : cancelled());
    controller.signal.addEventListener("abort", abortListener, { once: true });
  });
  signal?.addEventListener("abort", abort, { once: true });
  const timer = setTimeout(() => {
    timedOut = true;
    controller.abort();
  }, timeoutMs);

  async function request() {
    const response = await fetchImpl(`${PRICE_API}/${encodeURIComponent(product)}.json`, {
      signal: controller.signal,
      headers: { Accept: "application/json" },
      credentials: "omit",
      cache: "no-store",
      redirect: "error",
    });
    if (controller.signal.aborted) {
      void response.body?.cancel().catch(() => {});
      throw cancelled();
    }
    if (!response.ok) {
      void response.body?.cancel().catch(() => {});
      throw new PriceRequestError("status", `The price service returned HTTP ${response.status}.`);
    }
    const declaredLength = response.headers.get("content-length");
    if (declaredLength && /^\d+$/.test(declaredLength) && Number(declaredLength) > maxBytes) {
      void response.body?.cancel().catch(() => {});
      throw new PriceRequestError("size", "The price response exceeds the 16 KiB limit.");
    }
    if (!response.body) return "";
    reader = response.body.getReader();
    const decoder = new TextDecoder("utf-8", { fatal: true });
    let bytes = 0;
    let body = "";
    try {
      while (true) {
        const chunk = await reader.read();
        if (controller.signal.aborted) throw cancelled();
        if (chunk.done) return body + decoder.decode();
        bytes += chunk.value.byteLength;
        if (bytes > maxBytes) {
          throw new PriceRequestError("size", "The price response exceeds the 16 KiB limit.");
        }
        body += decoder.decode(chunk.value, { stream: true });
      }
    } finally {
      void reader.cancel().catch(() => {});
      reader.releaseLock();
    }
  }

  try {
    return await Promise.race([request(), interrupted]);
  } catch (error) {
    if (error instanceof PriceRequestError) throw error;
    throw new PriceRequestError("network", "The price response could not be read. Check your connection and try again.");
  } finally {
    clearTimeout(timer);
    signal?.removeEventListener("abort", abort);
    controller.signal.removeEventListener("abort", abortListener);
    controller.abort();
    if (reader) void reader.cancel().catch(() => {});
  }
}
