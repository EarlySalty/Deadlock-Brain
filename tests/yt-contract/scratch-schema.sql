-- Isolierte Scratch-Schema-Fixture fuer die deadlock-brain-yt DB-Vertragstests.
-- Schemareiner pg_dump der brain.youtube_*-Tabellen (keine Daten, kein Owner/Privileg),
-- plus minimaler brain.source_documents fuer die FK. Nur fuer leere Wegwerf-Testdatenbanken.
CREATE SCHEMA IF NOT EXISTS brain;
CREATE TABLE brain.source_documents (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, url text);

--
-- PostgreSQL database dump
--

\restrict sR9pc7uco4Fnu05sHebiGhLo23zwhvAWxBo46ZRP32CVFsPPcH16zWhkHXc4TxS

-- Dumped from database version 16.14 (Ubuntu 16.14-1.pgdg24.04+1)
-- Dumped by pg_dump version 16.15 (Ubuntu 16.15-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: youtube_feed_sources; Type: TABLE; Schema: brain; Owner: -
--

CREATE TABLE brain.youtube_feed_sources (
    feed_key text NOT NULL,
    source_type text NOT NULL,
    url text NOT NULL,
    handle text,
    playlist_id text,
    channel_id text,
    title text,
    enabled bigint DEFAULT 1 NOT NULL,
    metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Name: youtube_learning_claims; Type: TABLE; Schema: brain; Owner: -
--

CREATE TABLE brain.youtube_learning_claims (
    id bigint NOT NULL,
    legacy_sqlite_id bigint,
    video_id text NOT NULL,
    claim_hash text NOT NULL,
    claim_index bigint NOT NULL,
    entity_type text,
    entity_name text,
    claim_type text NOT NULL,
    claim_text text NOT NULL,
    evidence_quote text NOT NULL,
    timestamp_seconds double precision,
    model_confidence double precision NOT NULL,
    verifier_confidence double precision NOT NULL,
    status text NOT NULL,
    model text,
    prompt_version text NOT NULL,
    prompt_text text NOT NULL,
    model_response_text text NOT NULL,
    provider_metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
    verifier jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Name: youtube_learning_claims_id_seq; Type: SEQUENCE; Schema: brain; Owner: -
--

CREATE SEQUENCE brain.youtube_learning_claims_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: youtube_learning_claims_id_seq; Type: SEQUENCE OWNED BY; Schema: brain; Owner: -
--

ALTER SEQUENCE brain.youtube_learning_claims_id_seq OWNED BY brain.youtube_learning_claims.id;


--
-- Name: youtube_transcript_claim_attempts; Type: TABLE; Schema: brain; Owner: -
--

CREATE TABLE brain.youtube_transcript_claim_attempts (
    video_id text NOT NULL,
    prompt_version text NOT NULL,
    mode text DEFAULT 'normal'::text NOT NULL,
    status text NOT NULL,
    claim_count bigint DEFAULT 0 NOT NULL,
    char_len bigint DEFAULT 0 NOT NULL,
    note text,
    attempted_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Name: youtube_transcripts; Type: TABLE; Schema: brain; Owner: -
--

CREATE TABLE brain.youtube_transcripts (
    video_id text NOT NULL,
    language text,
    source_kind text NOT NULL,
    transcript_text text NOT NULL,
    content_hash text NOT NULL,
    source_document_id bigint,
    legacy_source_document_id bigint,
    imported_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Name: youtube_videos; Type: TABLE; Schema: brain; Owner: -
--

CREATE TABLE brain.youtube_videos (
    video_id text NOT NULL,
    feed_key text NOT NULL,
    channel_id text,
    channel_title text,
    title text NOT NULL,
    url text NOT NULL,
    published_at timestamp with time zone,
    description text,
    metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
    transcript_status text DEFAULT 'missing'::text NOT NULL,
    learning_status text DEFAULT 'queued'::text NOT NULL,
    discovered_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Name: youtube_learning_claims id; Type: DEFAULT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_learning_claims ALTER COLUMN id SET DEFAULT nextval('brain.youtube_learning_claims_id_seq'::regclass);


--
-- Name: youtube_feed_sources youtube_feed_sources_pkey; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_feed_sources
    ADD CONSTRAINT youtube_feed_sources_pkey PRIMARY KEY (feed_key);


--
-- Name: youtube_learning_claims youtube_learning_claims_claim_hash_key; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_learning_claims
    ADD CONSTRAINT youtube_learning_claims_claim_hash_key UNIQUE (claim_hash);


--
-- Name: youtube_learning_claims youtube_learning_claims_legacy_sqlite_id_key; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_learning_claims
    ADD CONSTRAINT youtube_learning_claims_legacy_sqlite_id_key UNIQUE (legacy_sqlite_id);


--
-- Name: youtube_learning_claims youtube_learning_claims_pkey; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_learning_claims
    ADD CONSTRAINT youtube_learning_claims_pkey PRIMARY KEY (id);


--
-- Name: youtube_transcript_claim_attempts youtube_transcript_claim_attempts_pkey; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_transcript_claim_attempts
    ADD CONSTRAINT youtube_transcript_claim_attempts_pkey PRIMARY KEY (video_id, prompt_version);


--
-- Name: youtube_transcripts youtube_transcripts_pkey; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_transcripts
    ADD CONSTRAINT youtube_transcripts_pkey PRIMARY KEY (video_id);


--
-- Name: youtube_videos youtube_videos_pkey; Type: CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_videos
    ADD CONSTRAINT youtube_videos_pkey PRIMARY KEY (video_id);


--
-- Name: brain_youtube_learning_claims_entity_idx; Type: INDEX; Schema: brain; Owner: -
--

CREATE INDEX brain_youtube_learning_claims_entity_idx ON brain.youtube_learning_claims USING btree (entity_type, entity_name);


--
-- Name: brain_youtube_learning_claims_video_idx; Type: INDEX; Schema: brain; Owner: -
--

CREATE INDEX brain_youtube_learning_claims_video_idx ON brain.youtube_learning_claims USING btree (video_id, status);


--
-- Name: brain_youtube_transcript_claim_attempts_prompt_status_idx; Type: INDEX; Schema: brain; Owner: -
--

CREATE INDEX brain_youtube_transcript_claim_attempts_prompt_status_idx ON brain.youtube_transcript_claim_attempts USING btree (prompt_version, status);


--
-- Name: brain_youtube_transcripts_source_document_idx; Type: INDEX; Schema: brain; Owner: -
--

CREATE INDEX brain_youtube_transcripts_source_document_idx ON brain.youtube_transcripts USING btree (source_document_id);


--
-- Name: brain_youtube_videos_feed_idx; Type: INDEX; Schema: brain; Owner: -
--

CREATE INDEX brain_youtube_videos_feed_idx ON brain.youtube_videos USING btree (feed_key);


--
-- Name: brain_youtube_videos_learning_idx; Type: INDEX; Schema: brain; Owner: -
--

CREATE INDEX brain_youtube_videos_learning_idx ON brain.youtube_videos USING btree (learning_status, transcript_status, published_at);


--
-- Name: youtube_learning_claims youtube_learning_claims_video_id_fkey; Type: FK CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_learning_claims
    ADD CONSTRAINT youtube_learning_claims_video_id_fkey FOREIGN KEY (video_id) REFERENCES brain.youtube_videos(video_id);


--
-- Name: youtube_transcript_claim_attempts youtube_transcript_claim_attempts_video_id_fkey; Type: FK CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_transcript_claim_attempts
    ADD CONSTRAINT youtube_transcript_claim_attempts_video_id_fkey FOREIGN KEY (video_id) REFERENCES brain.youtube_videos(video_id);


--
-- Name: youtube_transcripts youtube_transcripts_source_document_id_fkey; Type: FK CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_transcripts
    ADD CONSTRAINT youtube_transcripts_source_document_id_fkey FOREIGN KEY (source_document_id) REFERENCES brain.source_documents(id) ON DELETE SET NULL;


--
-- Name: youtube_transcripts youtube_transcripts_video_id_fkey; Type: FK CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_transcripts
    ADD CONSTRAINT youtube_transcripts_video_id_fkey FOREIGN KEY (video_id) REFERENCES brain.youtube_videos(video_id);


--
-- Name: youtube_videos youtube_videos_feed_key_fkey; Type: FK CONSTRAINT; Schema: brain; Owner: -
--

ALTER TABLE ONLY brain.youtube_videos
    ADD CONSTRAINT youtube_videos_feed_key_fkey FOREIGN KEY (feed_key) REFERENCES brain.youtube_feed_sources(feed_key);


--
-- PostgreSQL database dump complete
--

\unrestrict sR9pc7uco4Fnu05sHebiGhLo23zwhvAWxBo46ZRP32CVFsPPcH16zWhkHXc4TxS

