-- Colleges table
CREATE TABLE colleges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    public_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    short_name VARCHAR(50),
    logo_url TEXT,
    website_url TEXT,
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100) NOT NULL DEFAULT 'India',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_colleges_slug ON colleges(slug);
CREATE INDEX idx_colleges_public_id ON colleges(public_id);
CREATE INDEX idx_colleges_active ON colleges(is_active) WHERE is_active = TRUE;

-- College email domains for auto-verification
CREATE TABLE college_domains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    college_id UUID NOT NULL REFERENCES colleges(id) ON DELETE CASCADE,
    domain VARCHAR(255) NOT NULL UNIQUE,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_college_domains_domain ON college_domains(domain);
CREATE INDEX idx_college_domains_college ON college_domains(college_id);

-- Academic affiliations (user <-> college relationship)
CREATE TABLE academic_affiliations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    public_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    college_id UUID NOT NULL REFERENCES colleges(id) ON DELETE CASCADE,
    program VARCHAR(200),
    department VARCHAR(200),
    year_start INTEGER,
    year_end INTEGER,
    status VARCHAR(30) NOT NULL DEFAULT 'student',
    -- status: student, graduated, alumni, suspended
    verification_level INTEGER NOT NULL DEFAULT 0,
    -- 0=unverified, 1=email_verified, 2=document_verified, 3=admin_verified
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, college_id)
);

CREATE INDEX idx_affiliations_user ON academic_affiliations(user_id);
CREATE INDEX idx_affiliations_college ON academic_affiliations(college_id);
CREATE INDEX idx_affiliations_status ON academic_affiliations(status);

-- Verification requests (manual verification queue)
CREATE TABLE verification_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    affiliation_id UUID NOT NULL REFERENCES academic_affiliations(id) ON DELETE CASCADE,
    verification_type VARCHAR(50) NOT NULL,
    -- email, student_id, document, manual
    status VARCHAR(30) NOT NULL DEFAULT 'pending',
    -- pending, approved, rejected
    document_url TEXT,
    reviewer_notes TEXT,
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_verification_requests_status ON verification_requests(status);
CREATE INDEX idx_verification_requests_user ON verification_requests(user_id);
