\set ON_ERROR_STOP on
SELECT format('DROP DATABASE IF EXISTS %I WITH (FORCE)', :'db') WHERE :'db' LIKE 'brain\_pilot%' \gexec
SELECT format('CREATE DATABASE %I OWNER brain_migrate ENCODING ''UTF8'' TEMPLATE template0', :'db')
WHERE :'db' LIKE 'brain\_pilot%' \gexec
SELECT format('REVOKE ALL ON DATABASE %I FROM PUBLIC', :'db') \gexec
SELECT format('GRANT CONNECT ON DATABASE %I TO brain_ingest, brain_service, brain_readonly', :'db') \gexec
\connect :"db"
REVOKE ALL ON SCHEMA public FROM PUBLIC;
ALTER SCHEMA public OWNER TO brain_migrate;
CREATE SCHEMA IF NOT EXISTS brain AUTHORIZATION brain_migrate;
REVOKE ALL ON SCHEMA brain FROM PUBLIC;
