# Digital Campus

> Your campus. Your community. Your identity — on your terms.

A verified-but-anonymous college social network where students can participate anonymously or publicly, join college-specific and cross-college communities, and maintain a permanent account through graduation and alumni life.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust (2024 edition) + Axum + Tokio |
| Database | PostgreSQL 16 |
| Cache/PubSub | Redis 7 |
| Frontend | Next.js 16 + React 19 + TypeScript + Tailwind CSS v4 |
| Object Storage | MinIO (dev) / S3-compatible (prod) |
| Containers | Docker + Docker Compose |

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable, 1.85+)
- [Node.js](https://nodejs.org/) (v20+)
- [Docker](https://docs.docker.com/get-docker/) & Docker Compose
- [SQLx CLI](https://crates.io/crates/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`

## Quick Start

### 1. Clone & Configure

```bash
git clone <repo-url>
cd Campus_Connect
cp .env.example .env
# Edit .env and set a real JWT_SECRET (64+ random characters)
```

### 2. Start Infrastructure

```bash
docker compose up -d
```

This starts PostgreSQL, Redis, and MinIO.

### 3. Run Database Migrations

```bash
cd backend
sqlx migrate run
```

### 4. Start Backend

```bash
cd backend
cargo run
```

The API server starts at `http://localhost:8080`.

Verify with:
```bash
curl http://localhost:8080/api/v1/health
```

### 5. Start Frontend

```bash
cd frontend
npm install
npm run dev
```

The web app starts at `http://localhost:3000`.

## Project Structure

```
Campus_Connect/
├── backend/               # Rust/Axum backend
│   ├── src/
│   │   ├── main.rs        # Entrypoint
│   │   ├── config/        # Environment configuration
│   │   ├── app/           # App state, router assembly
│   │   ├── auth/          # Authentication module
│   │   ├── users/         # User profiles
│   │   ├── colleges/      # College registry & verification
│   │   ├── communities/   # Community system
│   │   ├── identity/      # Anonymous identity system
│   │   ├── posts/         # Posts & content
│   │   ├── comments/      # Comments & replies
│   │   ├── votes/         # Voting system
│   │   ├── chat/          # Real-time messaging
│   │   ├── events/        # Events system
│   │   ├── clubs/         # Club system
│   │   ├── moderation/    # Content moderation
│   │   ├── notifications/ # Notification system
│   │   ├── search/        # Search functionality
│   │   ├── marketplace/   # Student marketplace
│   │   ├── reputation/    # Topic-aware reputation
│   │   ├── feed/          # Feed generation
│   │   ├── files/         # File uploads
│   │   ├── middleware/    # HTTP middleware
│   │   ├── db/            # Database utilities
│   │   └── errors/        # Error types
│   └── migrations/        # SQLx database migrations
├── frontend/              # Next.js frontend
│   └── src/
│       ├── app/           # App Router pages
│       ├── components/    # React components
│       ├── hooks/         # Custom hooks
│       ├── lib/           # Utilities & API client
│       ├── providers/     # Context providers
│       └── types/         # TypeScript types
├── docker-compose.yml     # Local dev infrastructure
├── .env.example           # Environment template
├── ARCHITECTURE.md        # Architecture documentation
├── SECURITY.md            # Security model
└── PRIVACY.md             # Privacy architecture
```

## Development

### Backend Commands

```bash
cd backend
cargo check          # Type check
cargo fmt            # Format code
cargo clippy         # Lint
cargo test           # Run tests
cargo run            # Start server
```

### Frontend Commands

```bash
cd frontend
npm run dev          # Development server
npm run build        # Production build
npm run lint         # Lint
npm run typecheck    # Type check (if configured)
```

### Database

```bash
cd backend
sqlx migrate run      # Apply migrations
sqlx migrate revert   # Revert last migration
sqlx migrate add <name>  # Create new migration
```

## API

Base URL: `http://localhost:8080/api/v1`

### Health Check
```
GET /api/v1/health        # Basic health
GET /api/v1/health/ready  # Full readiness (DB + Redis)
```

### Authentication
```
POST /api/v1/auth/register   # Create account
POST /api/v1/auth/login      # Sign in
POST /api/v1/auth/refresh    # Refresh tokens
POST /api/v1/auth/logout     # Sign out
```

See [API.md](API.md) for full API documentation.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

## Security

See [SECURITY.md](SECURITY.md) for the security model.

## Privacy

See [PRIVACY.md](PRIVACY.md) for the privacy architecture.

## License

Private — All rights reserved.
