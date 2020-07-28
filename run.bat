wasm-pack build --target web --out-name songday --out-dir ./pkg
copy /Y src\asset\index.html pkg
miniserve ./pkg --index index.html