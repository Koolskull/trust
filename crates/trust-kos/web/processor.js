/*
 *                    ☦
 *   Lord Jesus Christ, Son of God, have mercy on us.
 *   Come, Holy Spirit.
 *
 * AudioWorklet. The module arrives already compiled on the main thread.
 * No SharedArrayBuffer. Views are created once. 128-frame blocks.
 */

class TrustProcessor extends AudioWorkletProcessor {
  constructor() {
    super();
    this.ready = false;
    this.exports = null;
    this.phase = 0;
    this.port.onmessage = (event) => {
      const data = event.data || {};
      if (data.module) {
        WebAssembly.instantiate(data.module).then((instance) => {
          const exports = instance.exports;
          exports.trust_init(sampleRate);
          const memory = exports.memory.buffer;
          const frames = exports.trust_block_frames();
          this.exports = exports;
          this.frames = frames;
          this.ptrL = exports.trust_in_l();
          this.ptrR = exports.trust_in_r();
          this.ptrOL = exports.trust_out_l();
          this.ptrOR = exports.trust_out_r();
          this.inL = new Float32Array(memory, this.ptrL, frames);
          this.inR = new Float32Array(memory, this.ptrR, frames);
          this.outL = new Float32Array(memory, this.ptrOL, frames);
          this.outR = new Float32Array(memory, this.ptrOR, frames);
          this.ready = true;
        });
        return;
      }
      if (this.ready && data.param) {
        this.exports.trust_set_param(data.param | 0, data.value | 0);
      }
    };
  }

  process(_inputs, outputs) {
    if (!this.ready) {
      return true;
    }
    const output = outputs[0] || [];
    const frames = output[0] ? output[0].length : this.frames;
    let offset = 0;
    while (offset < frames) {
      const n = Math.min(this.frames, frames - offset);
      for (let i = 0; i < n; i++) {
        const s = Math.sin(this.phase) * 0.35;
        this.phase += (2 * Math.PI * 220) / sampleRate;
        this.inL[i] = s;
        this.inR[i] = s * 0.8;
      }
      this.exports.trust_process(this.ptrL, this.ptrR, this.ptrOL, this.ptrOR, n);
      for (let i = 0; i < n; i++) {
        if (output[0]) output[0][offset + i] = this.outL[i];
        if (output[1]) output[1][offset + i] = this.outR[i];
      }
      offset += n;
    }
    return true;
  }
}

registerProcessor("trust-krush", TrustProcessor);
