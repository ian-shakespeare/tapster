-- Add up migration script here

ALTER TABLE units
ALTER COLUMN abbreviation TYPE VARCHAR(64);

INSERT INTO unit_systems (name)
VALUES
  ('metric'),
  ('imperial');

INSERT INTO units (name, abbreviation, unit_system_id)
VALUES
  ('fluid ounce', 'oz', (SELECT unit_system_id FROM unit_systems WHERE name = 'imperial' LIMIT 1)),
  ('milliliter', 'ml', (SELECT unit_system_id FROM unit_systems WHERE name = 'metric' LIMIT 1)),
  ('barspoon', 'barspoon', NULL),
  ('dash', 'dash', NULL),
  ('liter', 'l', (SELECT unit_system_id FROM unit_systems WHERE name = 'metric' LIMIT 1)),
  ('cup', 'cup', (SELECT unit_system_id FROM unit_systems WHERE name = 'imperial' LIMIT 1)),
  ('milligram', 'mg', (SELECT unit_system_id FROM unit_systems WHERE name = 'metric' LIMIT 1)),
  ('gram', 'g', (SELECT unit_system_id FROM unit_systems WHERE name = 'metric' LIMIT 1));
