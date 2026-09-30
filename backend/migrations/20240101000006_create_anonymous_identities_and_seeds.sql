-- Anonymous identities (community-scoped)
CREATE TABLE anonymous_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    community_id UUID, -- NULL = global, populated later when communities exist
    display_alias VARCHAR(100) NOT NULL DEFAULT 'Anonymous Student',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, community_id)
);

-- IMPORTANT: This index allows lookup by user+community but NEVER by community alone
-- to enumerate all anonymous identities. The unique constraint prevents duplicates.
CREATE INDEX idx_anon_identities_user ON anonymous_identities(user_id);
-- NO index on community_id alone: prevents "find all anon users in community" queries

-- Seed sample Indian colleges
INSERT INTO colleges (name, slug, short_name, city, state) VALUES
    ('Vellore Institute of Technology', 'vit-vellore', 'VIT', 'Vellore', 'Tamil Nadu'),
    ('Birla Institute of Technology and Science', 'bits-pilani', 'BITS', 'Pilani', 'Rajasthan'),
    ('Indian Institute of Technology Delhi', 'iit-delhi', 'IIT Delhi', 'New Delhi', 'Delhi'),
    ('Indian Institute of Technology Bombay', 'iit-bombay', 'IIT Bombay', 'Mumbai', 'Maharashtra'),
    ('Indian Institute of Technology Madras', 'iit-madras', 'IIT Madras', 'Chennai', 'Tamil Nadu'),
    ('National Institute of Technology Trichy', 'nit-trichy', 'NIT Trichy', 'Tiruchirappalli', 'Tamil Nadu'),
    ('Indian Institute of Technology Kanpur', 'iit-kanpur', 'IIT Kanpur', 'Kanpur', 'Uttar Pradesh'),
    ('Manipal Institute of Technology', 'mit-manipal', 'MIT', 'Manipal', 'Karnataka'),
    ('SRM Institute of Science and Technology', 'srm-chennai', 'SRM', 'Chennai', 'Tamil Nadu'),
    ('Delhi Technological University', 'dtu-delhi', 'DTU', 'New Delhi', 'Delhi');

-- Seed college email domains
INSERT INTO college_domains (college_id, domain, is_primary)
SELECT c.id, d.domain, TRUE
FROM (VALUES
    ('vit-vellore', 'vit.ac.in'),
    ('vit-vellore', 'vitstudent.ac.in'),
    ('bits-pilani', 'pilani.bits-pilani.ac.in'),
    ('bits-pilani', 'bits-pilani.ac.in'),
    ('iit-delhi', 'iitd.ac.in'),
    ('iit-bombay', 'iitb.ac.in'),
    ('iit-madras', 'iitm.ac.in'),
    ('nit-trichy', 'nitt.edu'),
    ('iit-kanpur', 'iitk.ac.in'),
    ('mit-manipal', 'learner.manipal.edu'),
    ('srm-chennai', 'srmist.edu.in'),
    ('dtu-delhi', 'dtu.ac.in')
) AS d(slug, domain)
JOIN colleges c ON c.slug = d.slug;
