BEGIN;
CREATE TABLE IF NOT EXISTS brain.site_comments_v1 (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    group_key text NOT NULL CHECK (char_length(group_key) <= 40),
    text text NOT NULL CHECK (char_length(text) BETWEEN 1 AND 4000),
    ts text NOT NULL CHECK (char_length(ts) <= 40 AND ts !~ '[<>&]')
);
CREATE INDEX IF NOT EXISTS site_comments_v1_group_order ON brain.site_comments_v1 (group_key, id);
REVOKE ALL ON brain.site_comments_v1 FROM PUBLIC;
REVOKE ALL ON SEQUENCE brain.site_comments_v1_id_seq FROM PUBLIC;
CREATE FUNCTION brain.append_site_comment_v1(requested_key text, requested_text text, requested_ts text)
RETURNS TABLE(text text, ts text) LANGUAGE plpgsql SECURITY DEFINER
SET search_path = pg_catalog, brain
AS $function$
BEGIN
    IF requested_key IS NULL OR char_length(requested_key)>40 OR requested_key ~ '[[:cntrl:]]'
       OR requested_ts IS NULL OR char_length(requested_ts)>40 OR requested_ts ~ '[[:cntrl:]<>&]'
       OR (requested_text IS NOT NULL AND (char_length(requested_text) NOT BETWEEN 1 AND 4000 OR btrim(requested_text)='')) THEN
        RAISE EXCEPTION 'Ungültiger Kommentar';
    END IF;
    PERFORM pg_advisory_xact_lock(hashtextextended(requested_key,742110026113));
    IF requested_text IS NOT NULL THEN
        INSERT INTO brain.site_comments_v1(group_key,text,ts) VALUES(requested_key,requested_text,requested_ts);
    END IF;
    RETURN QUERY SELECT c.text,c.ts FROM brain.site_comments_v1 c WHERE c.group_key=requested_key ORDER BY c.id;
END
$function$;
REVOKE ALL ON FUNCTION brain.append_site_comment_v1(text,text,text) FROM PUBLIC;
REVOKE ALL ON brain.site_comments_v1 FROM brain_site;
REVOKE ALL ON SEQUENCE brain.site_comments_v1_id_seq FROM brain_site;
GRANT SELECT ON brain.site_comments_v1 TO brain_site;
GRANT EXECUTE ON FUNCTION brain.append_site_comment_v1(text,text,text) TO brain_site;
COMMIT;
