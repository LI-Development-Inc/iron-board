# Technical Specification

## Language

- Rust 2024 Edition

---

## Runtime

- Tokio async runtime

---

## Web Framework

- Axum

---

## Templating

- Askama (server-side rendering)

---

## Database

- SQLite
- rusqlite (bundled)
- Foreign keys enabled

---

## Authentication

- Argon2 password hashing
- UUID-based session tokens
- Role-based access control

---

## Data Model

### Boards

- id (UUID)
- name (unique)
- description
- created_at

### Threads

- id (UUID)
- board_id (FK)
- title
- created_at

### Posts

- id (UUID)
- thread_id (FK)
- content
- created_at

### Users

- id (UUID)
- username (unique)
- password_hash
- role

### Sessions

- token (UUID)
- user_id (FK)
- created_at

---

## Static Assets

- `/static/css/style.css`
- `/static/js/main.js`

Served via tower-http.

---

## Deployment Model

- Single binary
- SQLite file storage
- Optional Docker container
- Reverse proxy compatible

---

## Non-Goals (Lean v1)

- No plugin system
- No microservices
- No ORM
- No SPA framework
- No frontend build pipeline
