// game-worker.js — runs in a Web Worker, so Atomics.wait is allowed

let view = null;

self.onmessage = async (e) => {
  if (e.data.type === 'init') {
    view = new Int32Array(e.data.sab);

    self.alert = (msg) => {
      self.postMessage({ type: 'alert', msg: String(msg ?? '') });
    };

    self.prompt = (msg) => {
      // Reset signal BEFORE posting so there's no race with a fast reply
      Atomics.store(view, 0, 0);

      // Tell main thread we need input
      self.postMessage({ type: 'prompt', msg: String(msg ?? '') });

      // Block here until main thread writes answer + notifies
      Atomics.wait(view, 0, 0);

      const charCode = Atomics.load(view, 1);
      return charCode === 0 ? '' : String.fromCharCode(charCode);
    };

    const { default: init, start_adventure } = await import(e.data.wasmUrl);
    await init();
    start_adventure();

    self.postMessage({ type: 'done' });
  }

  // Main thread sends answer by writing to SAB directly and notifying —
  // this message path is just a backup trigger (Atomics.notify wakes the wait)
  if (e.data.type === 'answer') {
    const charCode = e.data.value.charCodeAt(0);
    Atomics.store(view, 1, charCode);
    Atomics.store(view, 0, 1);
    Atomics.notify(view, 0, 1);
  }
};