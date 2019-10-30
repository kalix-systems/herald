# heraldcore configuration file

`heraldcore` can be configured using a `toml` file. Currently the only setting is
`server_addr`, which specifies the address of the server that the client will
try to connect to.

The path of the configuration file can be set by setting the environment variable
`HERALDCORE_CONF` to the path of the configuration file. For example,

```
HERALDCORE_CONF = $HOME/.heraldcore_conf.toml
```

If `HERALDCORE_CONF` is not set, the default values will be used.

Currently, the only value that can be set is `server_addr`. The following is an
example configuration:

```toml
server_addr = "127.0.0.1:8080"
```

## Platform specific differences

For desktop build targets, `HERALDCORE_CONF` can be set at anytime and the path will be read
at runtime (note: it will still require a restart to take effect).

For Android and iOS builds, this environment variable must be set at compile time and changing it
will require recompiling the library.
