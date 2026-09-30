# Privacy Architecture — Digital Campus

## Core Principle

> Privacy is a first-class architectural requirement, not an afterthought.

Digital Campus is designed around the principle that students should control their identity exposure. The platform knows who a verified user is, but other users only see what the user chooses to share.

## Identity Layers

The platform maintains strict separation between identity layers:

### 1. Permanent Account Identity
- Internal UUID (never exposed in APIs)
- Public UUID (opaque, used in API responses)
- Account status (active, suspended, deactivated)
- Created/updated timestamps

### 2. Credential Identity
- Email address (for authentication only)
- Password hash (Argon2id, never stored as plaintext)
- OAuth tokens (if applicable)
- Email verification status
- **Never exposed** to other users

### 3. Verification Identity
- College membership records
- Verification documents (processed and discarded)
- Verification level (0-5)
- Academic affiliation details
- **Never exposed** in full to other users

### 4. Public Profile Identity
- Display name (user-chosen)
- Avatar
- Bio
- Visible academic affiliation (user-controlled)
- Reputation scores
- **User controls** what is visible

### 5. Anonymous Identity
- Community-scoped (different ID per community)
- Display alias (e.g., "Anonymous Student")
- Not linkable across communities by design
- Mapping to real user **only accessible** by platform safety admins

## Data Minimization

### Collection Minimization
- Only collect data necessary for the feature
- Verification documents: process and discard, don't store permanently
- Don't require real name — display name is user-chosen
- College email is optional (alternative verification paths exist)

### Storage Minimization
- Verification documents have retention policies
- Session data expires and is cleaned up
- Deleted content is soft-deleted, then permanently removed after retention period

### Exposure Minimization
- API responses never include internal IDs
- API responses for anonymous content never include user identity
- Search results don't reveal anonymous post authors
- Notifications use the posting identity (public or anonymous)
- WebSocket events use the posting identity

## What Is Never Exposed to Ordinary Users

- Real email addresses
- Phone numbers
- Student ID numbers
- Verification documents
- Internal user IDs
- IP addresses
- Internal anonymous identity → user mappings
- Private metadata (login times, session data)
- Moderation records of other users

## What Moderators Can See

Community moderators can see:
- Post/comment content
- Anonymous display name (e.g., "Anonymous Student")
- Report details
- Moderation history for the community

Community moderators **CANNOT** see:
- Real identity behind anonymous posts
- User's email
- User's verification documents
- User's other community memberships

## What Platform Safety Admins Can See

With proper authorization and audit logging:
- Anonymous identity → user mapping (for safety investigations)
- User's verification status
- User's moderation history across communities

Platform safety admins **CANNOT** see:
- User's password (hashed, not reversible)
- User's private messages (without a separate warrant/policy)

## Audit Trail

Every privileged identity lookup creates an audit log entry containing:
- Who performed the lookup
- What identity was looked up
- When the lookup occurred
- Justification/reason

Audit logs are immutable and retained according to retention policy.

## User Privacy Controls

Users can configure:
- Default posting identity (Public or Anonymous)
- Whether to show college name
- Whether to show academic program
- Whether to show year/batch
- Who can send direct messages (Everyone / Verified Students / College Members / Nobody)
- Whether their profile appears in search

## Data Portability

Users can:
- Export their data (posts, comments, profile)
- Deactivate their account
- Request account deletion (subject to retention policies for moderation data)

## Anonymity Promise

The platform makes this promise:

> "Your real identity is not publicly attached to anonymous activity."

The platform does NOT promise:
- Perfect anonymity against platform administrators
- Anonymity against law enforcement with proper legal process
- Protection against users who voluntarily reveal their own identity
- Protection against inference attacks based on content (e.g., writing style)

This is documented transparently to users during onboarding.

## Technical Safeguards

1. **Separate tables**: Anonymous identities are in a separate table from user profiles
2. **No JOIN by default**: Standard queries don't join anonymous identities with users
3. **DTO mapping**: All API responses go through explicit DTO mapping that strips internal fields
4. **Type safety**: Rust's type system prevents accidental exposure of internal types in API responses
5. **Compile-time checks**: SQLx compile-time query checking ensures queries return expected fields
6. **Automated tests**: 10 mandatory anonymity tests verify non-leakage
