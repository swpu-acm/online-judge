---
"algohub-server": patch:refactor
---

Refactor `surrealdb::sql::Thing` in user interface data structures.

- Use `UserRecordId` instead `surreal::sql::Thing` in structs.
- Remove `role` from `Account` definition.
- Refactor and re-export `ProblemDetail` in user interface.
