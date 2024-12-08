-- Add up migration script here
CREATE TABLE unit (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE recipe (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    author integer REFERENCES app_user(id) ON DELETE CASCADE,
    prep_time INTERVAL,
    cook_time INTERVAL,
    inactive_time INTERVAL,
    yield_quantity integer NOT NULL,
    yield_units integer REFERENCES unit(id),
    UNIQUE (title, author)
);

CREATE TABLE ingredient (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE recipe_ingredient (
    id SERIAL PRIMARY KEY,
    recipe integer REFERENCES recipe(id) ON DELETE CASCADE,
    ingredient integer REFERENCES ingredient(id),
    quantity integer NOT NULL,
    units integer REFERENCES unit(id),
    preparation TEXT
);

CREATE TABLE step (
    id SERIAL PRIMARY KEY,
    recipe integer REFERENCES recipe(id) ON DELETE CASCADE,
    ordinal integer NOT NULL,
    instruction TEXT NOT NULL,
    UNIQUE (recipe, ordinal)
);

CREATE TYPE t_ingredient AS (
    id integer,
    name TEXT
);

CREATE TYPE t_unit AS (
    id integer,
    name TEXT
);

CREATE TYPE t_recipe_ingredient AS (
    id integer,
    recipe_id integer,
    ingredient t_ingredient,
    quantity integer,
    units t_unit,
    preparation TEXT
);

CREATE TYPE t_step AS (
    id integer,
    recipe_id integer,
    ordinal integer,
    instruction TEXT
);

CREATE TYPE t_app_user AS (
    id integer,
    name VARCHAR(100)
);
