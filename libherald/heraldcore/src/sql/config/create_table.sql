CREATE TABLE IF NOT EXISTS config (
  -- current user id
  id TEXT PRIMARY KEY NOT NULL,
  -- colorscheme
  colorscheme INTEGER NOT NULL,
  chk_id INTEGER UNIQUE default(1),
  CONSTRAINT CHK_config_singlerow CHECK (chk_id = 1)
)