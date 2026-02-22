# Rusty-Board Lean v1

Rusty-Board Lean v1 is a modular monolith imageboard backend written in Rust.

It is designed for:

- Simplicity
- Maintainability
- Clear layered architecture
- Server-side rendering
- Single binary deployment
- Minimal production-grade dependencies

---

## 🚀 Features

- Boards
- Threads
- Posts
- User authentication (Argon2)
- Session management
- Role-based access control (User / Admin)
- Server-side rendering (Askama)
- SQLite storage
- Static file serving
- Admin-protected routes
- Integration test structure

---

## 🏗 Architecture

This project follows a strict layered modular monolith structure:

- `models` → Domain structures
- `storage` → SQLite persistence layer
- `services` → Business logic
- `auth` → Cryptographic authentication utilities
- `api` → HTTP transport layer (Axum)
- `config` → Application configuration
- `bin/rusty-board` → Composition root

---

## 🛠 Technology Stack

- Rust 2024 Edition
- Axum
- Askama
- rusqlite (bundled)
- Argon2
- Tokio
- Tower HTTP
- Tracing

---

## 🧱 Deployment

Rusty-Board compiles into a single binary.

It supports:

- Local development
- Docker deployment
- Reverse proxy deployment
- SQLite file storage

---

## ▶ Running

```bash
cargo run
```

Default server:

```bash
http://localhost:3000
```

---

## 🧪 Testing

```bash
cargo test
```

---

## 📂 Project Structure

```bash
See `ARCHITECTURE.md` for full details.
```

```bash
iron-board/
├── Cargo.toml              ## Root cargo workspace 
├──bin/ 
│   └── rusty-board/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs     
├──crates/
│   ├── models/
│   │    ├── Cargo.toml
│   │    └── src/
│   │        └── lib.rs     
│   ├── storage/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      
│   ├── services/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      
│   ├── auth/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      
│   ├── config/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      
│   └── api/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs      
├──templates/
│       ├── Cargo.toml
│       └── src/
├──static/
│       ├── Cargo.toml
│       └── src/
└──tests/ ***
```
