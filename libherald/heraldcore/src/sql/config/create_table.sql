CREATE TABLE IF NOT EXISTS config (
  -- current user id
  id TEXT PRIMARY KEY NOT NULL,
  -- name as a string
  name TEXT,
  -- profile picture
  profile_picture TEXT,
  -- colorscheme setting
  colorscheme INTEGER default(0) NOT NULL,
  -- user color
  color INTEGER NOT NULL,
  -- enforce that this table only has one row
  chk_id INTEGER UNIQUE default(1),
  CONSTRAINT CHK_config_singlerow CHECK (chk_id = 1)
)