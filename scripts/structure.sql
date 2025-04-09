-- Create user if it doesn't exist
DO
$$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'newsletter') THEN
        CREATE ROLE newsletter WITH LOGIN PASSWORD 'password' CREATEDB;
    END IF;
END
$$;