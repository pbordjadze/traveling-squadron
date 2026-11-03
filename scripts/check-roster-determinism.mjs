// Replays one game of an evolution run in the browser build (wasm) and natively,
// and checks both the state hash and the detected moments agree.
// usage: node scripts/check-roster-determinism.mjs results/evolve/power/run.json [gen] [game] [ticks]
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const [runPath, gen = '40', game = '3', ticks = '4000'] = process.argv.slice(2);
const run = JSON.parse(readFileSync(runPath, 'utf8'));
const { instance } = await WebAssembly.instantiate(readFileSync('web/motes.wasm'), {});
const x = instance.exports;
const g = run.generations[+gen];
const nums = [run.rules.length, ...run.rules, g.inds.length];
for (const i of g.inds) nums.push(i.clan, ...i.genome);
nums.push(g.inds.length, ...g.inds.map((_, k) => k));
const pf = x.in_f64(nums.length);
new Float64Array(x.memory.buffer, pf, nums.length).set(nums);
const names = new TextEncoder().encode(g.inds.map((i) => `${run.family_names[i.family]}-${i.id}`).join('\n'));
const pt = x.in_txt(names.length);
new Uint8Array(x.memory.buffer, pt, names.length).set(names);
x.init_custom(g.seeds[+game]);
x.step(+ticks);
const h = (BigInt(x.hash_hi() >>> 0) << 32n | BigInt(x.hash_lo() >>> 0)).toString(16).padStart(16, '0');
const pfeed = x.feed(100000);
const feed = new TextDecoder().decode(new Uint8Array(x.memory.buffer, pfeed, x.text_len()));
const wasmMoments = feed.split('\n').filter((l) => l.includes('★')).length;
const native = execFileSync('target/release/evolve', ['--replay', runPath, gen, game, ticks]).toString();
const nh = native.match(/hash ([0-9a-f]+)/)[1];
const nMoments = native.split('\n').filter((l) => l.startsWith('moment')).length;
console.log(`gen ${gen} game ${game} tick ${ticks}: wasm ${h} native ${nh} ${h === nh ? 'SAME' : 'DIFFERENT'}; moments wasm ${wasmMoments} native ${nMoments}`);
process.exit(h === nh && wasmMoments === nMoments ? 0 : 1);
