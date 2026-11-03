// Compares the wasm build's state hash with the native runner's for a few seeds.
// usage: node scripts/check-determinism.mjs [path/to/motes.wasm]
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const wasmPath = process.argv[2] ?? 'web/motes.wasm';
const { instance } = await WebAssembly.instantiate(readFileSync(wasmPath), {});
const x = instance.exports;
let ok = true;
for (const seed of [1, 7, 12345]) {
  for (const ticks of [500, 3000]) {
    x.init(seed, 42); x.step(ticks);
    const h = (BigInt(x.hash_hi() >>> 0) << 32n | BigInt(x.hash_lo() >>> 0)).toString(16).padStart(16, '0');
    const native = execFileSync('target/release/runner', ['--hash', String(seed), '--ticks', String(ticks)]).toString();
    const nh = native.match(/hash ([0-9a-f]+)/)[1];
    const same = nh === h;
    ok &&= same;
    console.log(`seed ${seed} tick ${ticks}: wasm ${h} native ${nh} ${same ? 'SAME' : 'DIFFERENT'}`);
  }
  // scrubbing must land on the identical state too
  x.init(seed, 42); x.step(1234); const a = x.hash_lo(); x.seek(300); x.seek(1234);
  const same = a === x.hash_lo(); ok &&= same;
  console.log(`seed ${seed}: scrub back and forth ${same ? 'SAME' : 'DIFFERENT'}`);
}
process.exit(ok ? 0 : 1);
