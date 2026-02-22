# Design Document

## Philosophy

Rusty-Board Lean v1 prioritizes:

- Clarity over abstraction
- Explicit structure over flexibility
- Maintainability over extensibility
- Minimal dependencies
- Clean separation of concerns

---

## Design Principles

### 1. Modular Monolith

All functionality lives in one deployable unit.

---

### 2. Strict Layering

Transport Layer → Services → Storage → Database

Each layer has a single responsibility.

---

### 3. Server-Side Rendering

All pages rendered using Askama.

No SPA.
No frontend framework.

---

### 4. Minimal JavaScript

JavaScript is optional and limited to progressive enhancement.

---

### 5. Security Model

- Argon2 for passwords
- UUID session tokens
- Role-based access control
- Foreign key enforcement
- Clear admin separation

---

### 6. Database Strategy

SQLite is used for:

- Simplicity
- Zero infrastructure
- Embedded deployment
- Development speed

---

### 7. Scalability Path

Future versions may introduce:

- Caching
- Pagination
- Media handling
- Moderation workflows
- Performance optimizations

Lean v1 intentionally avoids premature complexity.
