BEGIN;
CREATE TABLE brain.site_comments_v1 (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    group_key text NOT NULL CHECK (char_length(group_key) <= 40),
    text text NOT NULL CHECK (char_length(text) BETWEEN 1 AND 4000),
    ts text NOT NULL CHECK (char_length(ts) <= 40 AND ts !~ '[<>&]')
);
CREATE INDEX site_comments_v1_group_order ON brain.site_comments_v1 (group_key, id);
REVOKE ALL ON brain.site_comments_v1 FROM PUBLIC;
REVOKE ALL ON SEQUENCE brain.site_comments_v1_id_seq FROM PUBLIC;
COMMIT;
