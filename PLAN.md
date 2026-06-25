- Project scaffolding — cargo new, dependencies in Cargo.toml, .env, directory structure
- Database migrations — schema first, everything else follows
- Seed binary — get real data into the database early
- Enums — the foundation everything else references
AppState and config
Error handling
- Repository layer — starting with read-only queries
- Handlers and router — public endpoints first, admin second
- Dual-write logic last — it touches the most moving parts