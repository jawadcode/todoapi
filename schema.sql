-- DB must have pgcrypto extension as a prerequisite
-- Queries **MUST** be separated by two newlines as shown below

CREATE TABLE IF NOT EXISTS users (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	displayname TEXT,
	username TEXT,
	email TEXT,
	password TEXT,
	created_at BIGINT
);

CREATE TABLE IF NOT EXISTS categories (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id UUID,
	name TEXT,
	description TEXT,
	tsv TSVECTOR,
	created_at BIGINT,
	updated_at BIGINT,
	CONSTRAINT fk_user
		FOREIGN KEY(user_id)
			REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS todos (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id UUID,
	cat_id UUID,
	title TEXT,
	description TEXT,
	tsv TSVECTOR,
	completed BOOLEAN,
	created_at BIGINT,
	updated_at BIGINT,
	CONSTRAINT fk_user
		FOREIGN KEY(user_id)
			REFERENCES users(id),
	CONSTRAINT fk_category
		FOREIGN KEY(cat_id)
			REFERENCES categories(id)
);

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
ON categories FOR EACH ROW EXECUTE PROCEDURE
tsvector_update_trigger(tsv, 'pg_catalog.english', name, description);

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
ON todos FOR EACH ROW EXECUTE PROCEDURE
tsvector_update_trigger(tsv, 'pg_catalog.english', title, description);