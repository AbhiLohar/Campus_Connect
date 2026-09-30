# Architecture — Digital Campus

## Overview

Digital Campus is a **modular monolith** built with Rust/Axum on the backend and Next.js on the frontend. The architecture is designed for clean separation of concerns while maintaining deployment simplicity.

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    CLIENTS                               │
│  Next.js App (SSR/CSR) │ Future Mobile App │ PWA         │
└──────────────┬──────────────────────────────┬────────────┘
               │ HTTPS REST + WSS             │
┌──────────────▼──────────────────────────────▼────────────┐
│              RUST / AXUM BACKEND                         │
│                                                          │
│  ┌──────────────────────────────────────────────────┐    │
│  │              MIDDLEWARE STACK                     │    │
│  │  Request ID → CORS → Rate Limit → Security Hdrs │    │
│  └──────────────────────┬───────────────────────────┘    │
│                         │                                │
│  ┌──────────────────────▼───────────────────────────┐    │
│  │              HANDLER LAYER (thin)                │    │
│  │  Parse request → validate → delegate → respond   │    │
│  └──────────────────────┬───────────────────────────┘    │
│                         │                                │
│  ┌──────────────────────▼───────────────────────────┐    │
│  │              SERVICE LAYER (business logic)      │    │
│  │  Authorization → domain rules → orchestration    │    │
│  └──────────────────────┬───────────────────────────┘    │
│                         │                                │
│  ┌──────────────────────▼───────────────────────────┐    │
│  │              REPOSITORY LAYER (data access)      │    │
│  │  SQL queries → row mapping → result types        │    │
│  └──────────────────────┬───────────────────────────┘    │
└─────────────────────────┼────────────────────────────────┘
                          │
          ┌───────────────┼───────────────┐
          │               │               │
     ┌────▼────┐    ┌────▼────┐    ┌─────▼──────┐
     │PostgreSQL│    │  Redis  │    │Object Store│
     │ (Truth) │    │ (Cache) │    │ (S3/MinIO) │
     └─────────┘    └─────────┘    └────────────┘
```

## Key Architectural Decisions

### ADR-001: Modular Monolith

**Decision**: Single deployable binary with clean module boundaries.

**Rationale**: A monolith simplifies deployment, debugging, and development. Module boundaries are clean enough that services can be extracted later if scale demands it.

**Consequences**: All domain logic runs in one process. Cross-domain calls are simple function calls, not network calls.

### ADR-002: Handler → Service → Repository Pattern

**Decision**: Three-layer architecture for each domain module.

- **Handlers**: Parse HTTP request, validate input, delegate to service, format response.
- **Services**: Business logic, authorization, orchestration. No HTTP concerns.
- **Repositories**: Database queries. No business logic.

**Rationale**: Testability, maintainability, clear separation of concerns.

### ADR-003: Opaque Public IDs

**Decision**: Internal entities use UUIDv7 (time-ordered) for primary keys. A separate `public_id` (UUIDv4, random) is used in all API responses.

**Rationale**: Prevents enumeration attacks, doesn't leak creation order, and separates internal references from public-facing identifiers.

### ADR-004: Identity Abstraction Layer

**Decision**: Posts and comments reference a `posting_identity` abstraction rather than directly referencing users.

**Rationale**: This enables the anonymous identity system. A post's visible author can be either a public profile or an anonymous identity, without the API consumer knowing the underlying user.

### ADR-005: Community-Scoped Anonymous Identities

**Decision**: Anonymous identities are scoped to a specific community. A user gets a different anonymous identity in each community.

**Rationale**: Prevents cross-community identity correlation. A user's anonymous activity in one community cannot be linked to their activity in another community by other users.

### ADR-006: PostgreSQL Full-Text Search

**Decision**: Use PostgreSQL's built-in full-text search initially.

**Rationale**: Avoids introducing a separate search service (Elasticsearch) for the MVP. PostgreSQL FTS is sufficient for the expected scale. The search layer is abstracted so a dedicated search engine can be swapped in later.

### ADR-007: Redis for Infrastructure Concerns Only

**Decision**: Redis is used for rate limiting, session caching, WebSocket pub/sub, and ephemeral data. PostgreSQL remains the source of truth.

**Rationale**: Redis is excellent for fast, ephemeral operations but should not be the primary store for permanent data.

### ADR-008: JWT Access + Database Refresh Tokens

**Decision**: Short-lived JWT access tokens (15 min) + long-lived database-stored refresh tokens (7 days) with rotation.

**Rationale**: JWTs enable stateless verification for most requests. Refresh tokens in the database enable revocation and rotation for security.

## Module Boundaries

Each domain module is designed to be independently extractable:

| Module | Responsibility | Could become |
|--------|---------------|--------------|
| `auth` | Authentication, tokens, sessions | Auth service |
| `users` | User profiles, settings | — |
| `colleges` | College registry, verification | — |
| `identity` | Anonymous identity management | — |
| `communities` | Community CRUD, membership, roles | Community service |
| `posts` | Post CRUD, post types | Content service |
| `comments` | Comment CRUD, nesting | Content service |
| `votes` | Voting system | Content service |
| `chat` | Real-time messaging, WebSockets | Chat service |
| `notifications` | Notification creation/delivery | Notification service |
| `moderation` | Reports, actions, audit | Moderation service |
| `events` | Event management | — |
| `clubs` | Club system | — |
| `search` | Search queries | Search service |
| `reputation` | Reputation scoring | — |
| `marketplace` | Listings, transactions | Marketplace service |
| `files` | Upload/download | Media service |
| `feed` | Feed generation | Feed service |

## Data Flow Examples

### Anonymous Post Creation

```
Client                 Handler           Service           Repository
  │                      │                 │                   │
  ├─ POST /posts ───────►│                 │                   │
  │  {community_id,      │                 │                   │
  │   title, body,       │                 │                   │
  │   is_anonymous:true} │                 │                   │
  │                      ├─ validate ──►   │                   │
  │                      ├─ auth check ──► │                   │
  │                      │                 ├─ verify membership│
  │                      │                 ├─ get/create       │
  │                      │                 │  anonymous_identity│
  │                      │                 ├─ create post with │
  │                      │                 │  identity_type=    │
  │                      │                 │  "anonymous"       │
  │                      │                 │                   ├── INSERT post
  │                      │                 │                   │
  │                      │◄── PostResponse │                   │
  │◄── 201 {post}────────│   (no user_id!) │                   │
```

### Token Refresh Flow

```
Client                 Handler           Service           Repository
  │                      │                 │                   │
  ├─ POST /auth/refresh─►│                 │                   │
  │  {refresh_token}     │                 │                   │
  │                      ├─ validate ──►   │                   │
  │                      │                 ├─ hash token       │
  │                      │                 │                   ├── find session
  │                      │                 ├─ verify not       │
  │                      │                 │  expired/revoked  │
  │                      │                 │                   ├── revoke old session
  │                      │                 │                   ├── create new session
  │                      │                 ├─ create new       │
  │                      │                 │  access + refresh │
  │◄── 200 {tokens}──────│◄────────────────│                   │
```
