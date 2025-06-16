-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Bars

CREATE TABLE IF NOT EXISTS users (
  user_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS media (
  media_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  size BIGINT NOT NULL DEFAULT 0,
  mime_type VARCHAR(255) NOT NULL DEFAULT 'application/octet-stream',
  created_at TIMESTAMP NOT NULL DEFAULT now(),

  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS profiles (
  profile_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  first_name VARCHAR(64) NOT NULL CHECK(length(first_name) >= 2),
  last_name VARCHAR(64) NOT NULL CHECK(length(last_name) >= 2),
  email VARCHAR(255) NOT NULL UNIQUE CHECK(length(email) >= 5),

  media_id UUID REFERENCES media(media_id) ON DELETE SET NULL,
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bars (
  bar_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR(64) NOT NULL CHECK(length(name) >= 2),
  created_at TIMESTAMP NOT NULL DEFAULT now(),

  media_id UUID REFERENCES media(media_id) ON DELETE SET NULL,
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE
);

-- Inventory

CREATE TABLE IF NOT EXISTS unit_systems (
  unit_system_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR(64) NOT NULL CHECK(length(name) >= 2)
);

CREATE TABLE IF NOT EXISTS units (
  unit_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR(64) NOT NULL CHECK(length(name) >= 2),
  abbreviation VARCHAR(5) NOT NULL CHECK(length(abbreviation) >= 1),

  unit_system_id UUID REFERENCES unit_systems(unit_system_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS ingredients (
  ingredient_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR(64) NOT NULL CHECK(length(name) >= 2),
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),

  media_id UUID REFERENCES media(media_id) ON DELETE SET NULL,
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ingredient_ingredients (
  ingredient_ingredient_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  parts SMALLINT NOT NULL CHECK(parts > 0),

  ingredient_id UUID NOT NULL REFERENCES ingredients(ingredient_id) ON DELETE CASCADE,
  compound_ingredient_id UUID NOT NULL REFERENCES ingredients(ingredient_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bar_ingredients (
  bar_ingredient_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  quantity REAL NOT NULL DEFAULT 1.0,

  bar_id UUID NOT NULL REFERENCES bars(bar_id) ON DELETE CASCADE,
  ingredient_id UUID NOT NULL REFERENCES ingredients(ingredient_id) ON DELETE CASCADE,
  unit_id UUID NOT NULL REFERENCES units(unit_id) ON DELETE CASCADE
);

-- Menu

CREATE TABLE IF NOT EXISTS recipes (
  recipe_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR(64) NOT NULL CHECK(length(name) >= 2),
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),

  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  media_id UUID REFERENCES media(media_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS recipe_ingredients (
  recipe_ingredient_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  quantity REAL NOT NULL DEFAULT 1.0,

  recipe_id UUID NOT NULL REFERENCES recipes(recipe_id) ON DELETE CASCADE,
  ingredient_id UUID NOT NULL REFERENCES ingredients(ingredient_id) ON DELETE CASCADE,
  unit_id UUID NOT NULL REFERENCES units(unit_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bar_recipes (
  bar_recipe_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  description VARCHAR(255) NOT NULL CHECK(length(description) >= 2),
  created_at TIMESTAMP NOT NULL DEFAULT now(),

  bar_id UUID NOT NULL REFERENCES bars(bar_id) ON DELETE CASCADE,
  recipe_id UUID NOT NULL REFERENCES recipes(recipe_id) ON DELETE CASCADE
);
