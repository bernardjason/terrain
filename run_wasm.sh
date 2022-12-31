cp src/index.html target/wasm32-unknown-emscripten/debug/
cp resources/loading.png target/wasm32-unknown-emscripten/debug/
cd target/wasm32-unknown-emscripten/debug/
echo  "GO TO http://127.0.0.1:8000"
echo
python3 -m http.server 8000
