self.addEventListener("install", () => self.skipWaiting());
self.addEventListener("activate", (e) => e.waitUntil(self.clients.claim()));

self.addEventListener("fetch", (e) => {
  // Only intercept same-origin requests — skip fonts, external CDNs, etc.
  const url = new URL(e.request.url);
  if (url.origin !== self.location.origin) return;

  e.respondWith(
    fetch(e.request).then((res) => {
      // Only rewrite responses with a valid status code
      if (!res.ok && res.status === 0) return res;
      const headers = new Headers(res.headers);
      headers.set("Cross-Origin-Opener-Policy", "same-origin");
      headers.set("Cross-Origin-Embedder-Policy", "require-corp");
      headers.set("Cross-Origin-Resource-Policy", "cross-origin");
      return new Response(res.body, {
        status: res.status,
        statusText: res.statusText,
        headers,
      });
    }).catch(() => fetch(e.request))
  );
});