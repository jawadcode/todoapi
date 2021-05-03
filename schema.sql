CREATE TABLE IF NOT EXISTS users (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	displayname TEXT NOT NULL,
	username TEXT UNIQUE NOT NULL,
	email TEXT UNIQUE NOT NULL,
	password TEXT NOT NULL,
	created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS categories (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id UUID NOT NULL,
	name TEXT NOT NULL,
	description TEXT NOT NULL,
	tsv TSVECTOR,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL,
	CONSTRAINT fk_user
		FOREIGN KEY(user_id)
			REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS todos (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id UUID NOT NULL,
	cat_id UUID NOT NULL,
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	tsv TSVECTOR NOT NULL,
	completed BOOLEAN NOT NULL,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL,
	CONSTRAINT fk_user
		FOREIGN KEY(user_id)
			REFERENCES users(id),
	CONSTRAINT fk_category
		FOREIGN KEY(cat_id)
			REFERENCES categories(id)
);

DROP TRIGGER IF EXISTS tsvectorupdate ON categories;

DROP TRIGGER IF EXISTS tsvectorupdate ON todos;

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
ON categories FOR EACH ROW EXECUTE PROCEDURE
tsvector_update_trigger(tsv, 'pg_catalog.english', name, description);

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
ON todos FOR EACH ROW EXECUTE PROCEDURE
tsvector_update_trigger(tsv, 'pg_catalog.english', title, description);