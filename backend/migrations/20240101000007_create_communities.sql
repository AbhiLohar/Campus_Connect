-- Communities
CREATE TABLE communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    public_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT DEFAULT '',
    icon_url TEXT,
    banner_url TEXT,
    college_id UUID REFERENCES colleges(id) ON DELETE SET NULL,
    community_type VARCHAR(30) NOT NULL DEFAULT 'general',
    -- general, college, club, course, batch, global
    visibility VARCHAR(20) NOT NULL DEFAULT 'public',
    -- public, private, invite_only
    join_policy VARCHAR(20) NOT NULL DEFAULT 'open',
    -- open, request, invite_only
    member_count INTEGER NOT NULL DEFAULT 0,
    is_official BOOLEAN NOT NULL DEFAULT FALSE,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_communities_public_id ON communities(public_id);
CREATE INDEX idx_communities_slug ON communities(slug);
CREATE INDEX idx_communities_college ON communities(college_id);
CREATE INDEX idx_communities_type ON communities(community_type);
CREATE INDEX idx_communities_visibility ON communities(visibility);

-- Community members
CREATE TABLE community_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(30) NOT NULL DEFAULT 'member',
    -- owner, admin, moderator, member
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(community_id, user_id)
);

CREATE INDEX idx_community_members_community ON community_members(community_id);
CREATE INDEX idx_community_members_user ON community_members(user_id);
CREATE INDEX idx_community_members_role ON community_members(community_id, role);

-- Community rules
CREATE TABLE community_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    rule_number INTEGER NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(community_id, rule_number)
);

-- Add community_id FK to anonymous_identities now that communities table exists
ALTER TABLE anonymous_identities
    ADD CONSTRAINT fk_anon_identity_community
    FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;
