-- migrate:up
CREATE TABLE tasks (
	id VARCHAR(255) PRIMARY KEY,  -- ID column as string
	status VARCHAR(255)           -- Status column as string
);

-- migrate:down
DROP TABLE IF EXISTS tasks;
