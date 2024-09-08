CREATE TABLE todos (
    "user_id" INTEGER NOT NULL,
    "todo_id" INTEGER NOT NULL,
    "creation_dt" DATE NOT NULL,
    "active_status" BOOLEAN NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "set_dt" DATE NOT NULL,
    "color" TEXT NOT NULL,
    PRIMARY KEY (user_id, todo_id)
);

CREATE TABLE users (
    "email" VARCHAR(40) NOT NULL, 
    "user_id" INTEGER NOT NULL, PRIMARY KEY(user_id),
    "user_name" VARCHAR(30) NOT NULL,
    "salt_hash_bytes" BYTEA NOT NULL,
    "jwt_code" VARCHAR NOT NULL,
    "recover_code" INTEGER NOT NULL,
    "recover_code_exp" DATE NOT NULL
);

CREATE TABLE colors (
    "color_hex" VARCHAR NOT NULL, PRIMARY KEY(color_hex),
    "color_name" VARCHAR NOT NULL
);

INSERT INTO colors ("color_hex", "color_name") VALUES
    ('#6f2da8', 'Grape'),
    ('#e30b5c', 'Raspberry'),
    ('#464196', 'Blueberry'),
    ('#ffee00', 'Banana'),
    ('#76cd26', 'Green Apple'),
    ('#f28500', 'Tangerine'),
    ('#965a3e', 'Coconut'),
    ('#b62625', 'Cherry'),
    ('#fb2943', 'Strawberry'),
    ('#7a960f', 'Kiwi');
