# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a peer group grading application built with:

**Backend**: Rust using Actix Web framework
- Located in `crates/backend/`
- Uses SeaORM for database operations with PostgreSQL
- Redis for session management
- OpenAPI documentation via utoipa
- JWT/session-based authentication
- LDAP integration support (currently disabled)

**Frontend**: Vue 3 with TypeScript
- Located in `frontend/`
- Uses Vite for build tooling
- PrimeVue for UI components
- Pinia for state management
- Vue Router for routing

**Database**: PostgreSQL with SeaORM migrations
- Migration crate in `crates/migration/`
- Entities defined in `crates/backend/src/db/entity/`

**Architecture Pattern**: 
- Rust workspace with multiple crates
- Monorepo structure with frontend and backend
- Docker Compose for local development
- REST API with OpenAPI/Swagger documentation

## Development Commands

### Backend (Rust)
```bash
# Build backend
cargo build --package backend

# Run backend with hot reload
cargo run --package backend

# Run backend tests
cargo test --package backend

# Run migrations
cargo run --package migration

# Run specific test
cargo test --package backend test_name

# Lint backend code
cargo clippy --package backend -- -W clippy::all
```

### Frontend (Vue)
```bash
# Navigate to frontend directory first
cd frontend

# Install dependencies
pnpm install

# Development server
pnpm dev

# Build for production
pnpm build

# Run tests
pnpm test:unit

# Type checking
pnpm type-check

# Lint and format
pnpm lint
pnpm format
```

### Docker Development
```bash
# Start all services (PostgreSQL, Redis, OpenLDAP, Backend)
docker compose -f dev-compose.yml up -d

# View logs
docker compose -f dev-compose.yml logs -f

# Stop services
docker compose -f dev-compose.yml down
```

## Key Components

### Backend Controllers
- `auth.rs` - Authentication and session management
- `user.rs` - User management endpoints
- `project.rs` - Project CRUD operations
- `group.rs` - Group management
- `class.rs` - Class/course management
- `template.rs` - Template management

### Database Entities
Located in `crates/backend/src/db/entity/`:
- `user.rs` - User entity
- `project.rs` - Project entity
- `group.rs` - Group entity
- `local_auth.rs` - Local authentication
- `user_group_project.rs` - Many-to-many relationships

### Frontend Structure
- `src/views/` - Vue page components
- `src/components/` - Reusable Vue components
- `src/stores/` - Pinia state management
- `src/router/` - Vue Router configuration

## Environment Configuration

Required environment variables (create `.env` file):
```
# Database
DB_HOST=localhost
DB_USER=pgg
DB_PASSWORD=pgg
DB_NAME=pgg
DB_PORT=5432

# Redis
REDIS_HOST=localhost
REDIS_PORT=6379

# LDAP (optional)
LDAP_ADMIN_PASSWORD=admin
```

## Testing

Backend uses:
- `testcontainers` for integration tests with real PostgreSQL/Redis
- `serial_test` for test isolation
- `temp-env` for environment variable testing

Frontend uses:
- Vitest for unit testing
- Vue Test Utils for component testing

## API Documentation

Swagger UI available at: `http://localhost:8080/swagger-ui/`
OpenAPI spec at: `http://localhost:8080/api-docs/openapi.json`

## Project Management Tools

Bruno API collection available in `bruno/` directory for API testing.