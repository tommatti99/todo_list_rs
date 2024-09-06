CREATE TABLE todos (
    "user_id" INTEGER NOT NULL, PRIMARY KEY (user_id, todo_id),
    "todo_id" INTEGER NOT NULL,
    "creation_dt" DATE NOT NULL,
    "active_status" BOOLEAN NOT NULL,
    "description" TEXT NOT NULL,
    "set_dt" DATE NOT NULL,
    "color" TEXT NOT NULL
);

CREATE TABLE users (
    "email" VARCHAR(40) NOT NULL, 
    "user_id" SMALLINT NOT NULL, PRIMARY KEY(user_id),
    "user_name" VARCHAR(30) NOT NULL,
    "salt_hash_bytes" BYTEA NOT NULL
);