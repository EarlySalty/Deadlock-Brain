CREATE TABLE IF NOT EXISTS patchnotes.changelog_posts (
    id BIGINT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    posted_at TIMESTAMPTZ,
    raw_content TEXT,
    translated_content TEXT
);

CREATE INDEX IF NOT EXISTS changelog_posts_posted_at_idx
    ON patchnotes.changelog_posts (posted_at DESC);

