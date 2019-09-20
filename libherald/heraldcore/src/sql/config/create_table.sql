CREATE TABLE IF NOT EXISTS config (
  -- current user id
  id TEXT PRIMARY KEY NOT NULL CHECK (length(id) <= 255),
  -- colorscheme
  colorscheme INTEGER NOT NULL,
  kp BLOB NOT NULL,
  -- enforce this table having no more than one row
  chk_id INTEGER UNIQUE default(1),
  CONSTRAINT CHK_config_singlerow CHECK (chk_id = 1)
)