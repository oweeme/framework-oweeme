#!/usr/bin/env bash
set -e

echo "=== Framework Oweeme — Build de producción ==="

# 1. Binario del servidor
echo "[1/3] Compilando servidor Rust..."
cargo build --release
echo "      → target/release/framework_oweeme"

# 2. WASM (requiere wasm-pack instalado: cargo install wasm-pack)
if command -v wasm-pack &>/dev/null; then
  echo "[2/3] Compilando módulo WASM..."
  cd wasm
  wasm-pack build --target web --out-dir ../static/wasm --release
  cd ..
  echo "      → static/wasm/oweeme_wasm.js"
else
  echo "[2/3] wasm-pack no encontrado. Instala con: cargo install wasm-pack"
  echo "      Saltando compilación WASM..."
fi

# 3. Empaqueta todo para despliegue
echo "[3/3] Empaquetando..."
mkdir -p dist
cp target/release/framework_oweeme dist/
cp -r templates dist/
cp -r static dist/
cp -r locales dist/
cp .env.example dist/.env
echo "      → dist/ listo para desplegar"

echo ""
echo "Para correr en producción:"
echo "  cd dist && ./framework_oweeme"
