#!/usr/bin/env bash
# Builds the wasm simulation and the single-file viewer (web/motes-viewer.html,
# which opens straight from disk: the wasm is embedded).
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release -p motes-wasm --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/motes_wasm.wasm web/motes.wasm
python3 - <<'PY'
import base64
src = open("web/viewer.html").read()
b64 = base64.b64encode(open("web/motes.wasm", "rb").read()).decode()
out = src.replace("/*WASM_BASE64*/", b64)
open("web/motes-viewer.html", "w").write("<!doctype html>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width,initial-scale=1,viewport-fit=cover\">\n" + out)
PY
echo "built web/motes-viewer.html ($(du -k web/motes-viewer.html | cut -f1) KB)"
