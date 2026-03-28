// game-worker.js
// Runs inside a Web Worker — Atomics.wait is allowed here.

let sab = null;       // SharedArrayBuffer: index 0 = signal, index 1 = answer char code
let view = null;

self.onmessage = async (e) => {
  if (e.data.type === 'init') {
    sab  = e.data.sab;
    view = new Int32Array(sab);

    // Intercept alert/prompt before loading Wasm
    // Workers don't have alert/prompt natively — we define them as postMessage calls.
    self.alert = (msg) => {
      self.postMessage({ type: 'alert', msg: String(msg ?? '') });
    };

    self.prompt = (msg) => {
      // Flush: tell main thread we need input
      self.postMessage({ type: 'prompt', msg: String(msg ?? '') });

      // Block this worker thread until main thread writes the answer
      Atomics.store(view, 0, 0);
      Atomics.wait(view, 0, 0);   // blocks until main thread calls Atomics.notify

      const charCode = Atomics.load(view, 1);
      return charCode === 0 ? '' : String.fromCharCode(charCode);
    };

    // Now import and run the Wasm module
    const { default: init, start_adventure } = await import(e.data.wasmUrl);
    await init();
    start_adventure();

    self.postMessage({ type: 'done' });
  }

  if (e.data.type === 'answer') {
    // Main thread is sending the user's choice
    const charCode = e.data.value.charCodeAt(0);
    Atomics.store(view, 1, charCode);
    Atomics.store(view, 0, 1);
    Atomics.notify(view, 0, 1);
  }
};