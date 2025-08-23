** Notes for Understanding

1. Setup Rust Project (Git, removed master branch)
2. Add Actix Web to Project
    * cargo add actix-web
3. Setup Project Folder Structure

    actix-react-postgres/
    ├── Cargo.toml                 # Rust dependencies and project metadata
    ├── .env                       # Environment variables (database URL, etc.)
    ├── migrations/                # Database migration files
    │   └── initial.sql
    ├── src/
    │   ├── main.rs               # Application entry point
    │   ├── lib.rs                # Library root for shared code
    │   ├── config/               # Configuration management
    │   │   └── mod.rs
    │   ├── database/             # Database connection and setup
    │   │   ├── mod.rs
    │   │   └── connection.rs
    │   ├── models/               # Data structures and database models
    │   │   ├── mod.rs
    │   │   └── user.rs
    │   ├── handlers/             # HTTP request handlers (controllers)
    │   │   ├── mod.rs
    │   │   └── user_handlers.rs
    │   ├── routes/               # Route definitions
    │   │   ├── mod.rs
    │   │   └── user_routes.rs
    │   ├── services/             # Business logic layer
    │   │   ├── mod.rs
    │   │   └── user_service.rs
    │   └── utils/                # Utility functions and helpers
    │       ├── mod.rs
    │       └── errors.rs
    └── frontend/                 # React application
        ├── package.json
        ├── src/
        │   ├── components/
        │   ├── services/
        │   └── App.js

4. Dependencies Added, .env added to gitignore

5. .env Updated

6. migrations/initial.sql updated for basic users table

7. config/mod.rs setup

8. database/connection.rs setup