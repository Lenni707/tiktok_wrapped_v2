# TikTok Wrapped (Rust + WebAssembly + React)

This project uses **Rust (compiled to WebAssembly)** for data processing and **React (Vite)** for the UI.

---

## 🚀 Setup

### 1. Install wasm-pack

```bash
cargo install wasm-pack
```

---

### 2. Build the WebAssembly module

From the root directory:

```bash
wasm-pack build --target bundler
```

This generates the `pkg/` folder used by the frontend.

---

### 3. Setup frontend

```bash
cd frontend
npm install
```

---

### 4. Link Rust → React

```bash
npm install ../pkg
```

---

### 5. Start development server

```bash
npm run dev
```

Then open:

```
http://localhost:5173
```

---

## 🔁 Development Workflow

Whenever you change Rust code:

```bash
wasm-pack build --target bundler
```

Then refresh the browser.

---

## 📁 Project Structure

```
tiktok_wrapped_v2/
├── src/              # Rust source code
├── pkg/              # Generated Wasm package (build output)
├── frontend/         # React (Vite) app
└── Cargo.toml
```

---

## ⚠️ Notes

* `pkg/`, `target/`, and `frontend/node_modules/` are generated — do not commit them
* WebAssembly runs in the browser, so file access must go through the frontend
* JSON parsing is handled in Rust using `serde_json`

---

## 🧠 How it works

```
User uploads JSON
        ↓
React reads file → string
        ↓
Rust (Wasm) parses JSON
        ↓
Returns structured data
        ↓
React renders UI

```

## LICENSED under MIT
```
