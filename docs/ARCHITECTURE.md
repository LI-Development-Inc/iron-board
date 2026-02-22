# Architecture Overview

Rusty-Board Lean v1 follows a modular monolith design.

It is NOT:

- A microservice system
- A plugin engine
- A frontend SPA platform
- A framework-based ecosystem

It is a clean backend application.

---

## Layered Structure

### 1. models

Pure domain structures only.
No logic.
No database.
No HTTP.

---

### 2. storage

SQLite persistence layer.

Responsibilities:

- Schema creation
- Connection handling
- Repositories
- No business logic

---

### 3. services

Business logic layer.

Responsibilities:

- Validation
- UUID generation
- Timestamp creation
- Rule enforcement

Must not contain SQL.

---

### 4. auth

Authentication utilities.

Responsibilities:

- Password hashing (Argon2)
- Password verification
- Session token generation
- Role utilities

---

### 5. api

HTTP layer (Axum).

Responsibilities:

- Routing
- Request parsing
- Response rendering
- Template integration

Must not contain business rules.

---

### 6. config

Application configuration.

Responsibilities:

- Environment loading
- Defaults
- Server/database configuration

---

### 7. bin/rusty-board

Composition root.

Responsibilities:

- Wiring dependencies
- Starting server
- Initializing database
- Loading configuration

---

## Request Flow

Browser
→ Axum Route
→ Service Layer
→ Storage Layer
→ SQLite
→ Askama Template
→ Response
