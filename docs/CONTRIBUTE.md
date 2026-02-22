# Contributing to Rusty-Board Lean v1

Thank you for considering contributing.

---

## Development Principles

All contributions must:

- Preserve layered architecture
- Avoid introducing new frameworks
- Avoid ORMs
- Avoid microservices
- Maintain single-binary compatibility

---

## Code Standards

- Use rustdoc on all public items
- Include TODO comments where appropriate
- Include developer notes at top of files
- Keep business logic inside `services`
- Keep SQL inside `storage`
- Keep HTTP inside `api`

---

## Testing

- Add unit tests for new logic
- Add integration tests when modifying system behavior
- Ensure `cargo test` passes

---

## Pull Request Requirements

Before submitting:

- Run `cargo fmt`
- Run `cargo clippy`
- Run `cargo test`
- Ensure project builds cleanly

---

## Architecture Integrity

Do NOT:

- Add plugins
- Introduce external frameworks
- Add unnecessary abstraction layers
- Break separation of concerns

---

## Thank You

Lean v1 is intentionally simple.

Please keep it that way.
