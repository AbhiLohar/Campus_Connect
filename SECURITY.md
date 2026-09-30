# Security Model — Digital Campus

## Overview

Security is a first-class architectural concern. This document describes the security model, threat mitigations, and implementation details.

## Authentication

### Password Storage
- **Algorithm**: Argon2id (memory-hard, resistant to GPU/ASIC attacks)
- **Parameters**: Default Argon2id parameters from the `argon2` crate
- Passwords are NEVER stored in plaintext or reversible encryption

### Token Architecture
- **Access Token**: JWT, 15-minute expiry, stateless verification
  - Contains: user public_id (opaque UUID), issued-at, expiry, token type
  - Does NOT contain: email, internal user ID, or other PII
- **Refresh Token**: Random token, 7-day expiry, stored in database (hashed)
  - Rotation: New refresh token issued on each refresh, old token revoked
  - Revocation: Explicit logout revokes the refresh token
  - Abuse detection: If a revoked refresh token is reused, all user sessions are revoked

### Session Management
- Refresh tokens are hashed (SHA-256) before storage
- Sessions track: user_id, token_hash, expiry, revocation status, creation time
- Expired sessions are periodically cleaned up

## Authorization

### Principle of Least Privilege
- Every API endpoint validates permissions server-side
- Frontend authorization is for UX only — never trusted
- Community operations check membership and role
- Moderation operations check moderator/admin role
- Identity lookups check platform-level safety admin role

### Role Hierarchy
```
Platform Admin > Community Owner > Community Admin > Community Moderator > Member > Visitor
```

## Input Validation

### Server-Side Validation
- All request bodies validated using the `validator` crate with derive macros
- Email format validation
- Password minimum length enforcement (8 characters)
- Display name length bounds (2-50 characters)
- Content length limits on all text fields
- URL format validation where applicable

### SQL Injection Protection
- All database queries use parameterized queries via SQLx
- No string interpolation in SQL
- Compile-time query checking where practical

### XSS Protection
- All API responses are JSON (Content-Type: application/json)
- Frontend uses React, which auto-escapes rendered content
- Security headers: `X-Content-Type-Options: nosniff`
- Content Security Policy headers in production

## HTTP Security Headers

Applied to all responses:
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Strict-Transport-Security: max-age=63072000` (production only)

## CORS

- Explicitly configured allowed origins (not wildcard)
- Credentials: allowed
- Methods: GET, POST, PATCH, DELETE, OPTIONS
- Exposed headers: X-Request-Id

## Rate Limiting

### Implementation
- Redis-backed sliding window rate limiter
- Per-user rate limiting (authenticated requests)
- Per-IP rate limiting (unauthenticated requests)
- Shared network awareness: rate limits account for college networks sharing IPs

### Default Limits
- General API: 10 requests/second, burst of 30
- Authentication endpoints: Stricter limits (5 attempts/minute)
- Registration: 3 accounts per IP per hour
- Content creation: 30 posts per hour per user

### Brute-Force Protection
- Login attempts tracked per email
- Progressive delay after failed attempts
- Account lockout after 10 consecutive failures (30-minute cooldown)
- Failed attempt logging for security audit

## File Upload Security

- MIME type validation (allowlist of safe types)
- File extension validation
- Maximum file size enforcement (10MB default, configurable)
- Files stored in object storage (S3-compatible), never in the database
- Private files accessed via signed URLs with expiration
- Content scanning can be added via the moderation module

## Anonymous Identity Security

### Threat Model

| Threat | Mitigation |
|--------|-----------|
| API leaks real identity | API serialization layer maps to DTOs that exclude user_id for anonymous content |
| IDOR reveals author | Post endpoints never accept or return user_id for anonymous posts |
| Moderator abuse | Moderators cannot reveal anonymous identities; only platform safety admins can |
| Cross-community correlation | Anonymous identities are community-scoped (different ID per community) |
| Notification leaks | Notifications about anonymous posts use anonymous display name |
| Search leaks | Search indexes use posting identity, not user identity |
| WebSocket leaks | WS events for anonymous posts use anonymous identity only |
| Database breach | Even with full DB access, anonymous_identity → user mapping is only in the anonymous_identities table, which should have restricted access |
| Timing attacks | Post creation time is the same regardless of identity type |
| Admin abuse | Every identity lookup is logged in audit_logs with justification |

### Mandatory Anonymity Tests

The following automated tests MUST pass:
1. User A cannot discover the real user behind User B's anonymous post via any API call
2. GET /posts/:id never returns author user_id for anonymous posts
3. Community moderator endpoints don't expose anonymous user identity
4. Anonymous identities cannot be correlated across communities
5. API serialization doesn't accidentally include user_id in responses
6. Search results don't leak anonymous identity
7. Notifications don't expose hidden identity
8. WebSocket events don't expose hidden identity
9. Application logs don't contain unnecessary identity data
10. Admin identity lookup is permission-controlled and creates audit log entry

## Audit Logging

### What Is Logged
- All authentication events (login, logout, failed attempts)
- All moderation actions
- All identity lookups by safety admins
- Session creation/revocation
- Role changes
- Account status changes

### What Is NOT Logged
- Passwords (even hashed)
- Full request/response bodies containing PII
- Private message content
- JWT token values

## Error Handling

### Production Mode
- API errors return structured JSON: `{ "error": { "code": "...", "message": "..." } }`
- Database errors, SQL errors, and stack traces are NEVER exposed to clients
- Internal errors return HTTP 500 with generic message

### Development Mode
- Additional diagnostic information may be included in error responses
- Tracing output includes request details

## Dependency Security

- Dependencies pinned to specific versions in Cargo.toml
- Regular dependency auditing via `cargo audit`
- Minimal dependency surface area
