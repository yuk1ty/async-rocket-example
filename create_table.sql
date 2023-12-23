CREATE TABLE IF NOT EXISTS todos (
	id uuid NOT NULL DEFAULT gen_random_uuid(),
	title varchar(256) NOT NULL,
	description varchar(4096),
	done boolean NOT NULL,
	creation_date timestamp NOT NULL DEFAULT now(),
	modified_date timestamp DEFAULT now(),
	CONSTRAINT todos_pk PRIMARY KEY (id)
)
