# Setting up diesel

Install the `diesel_cli`:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Note: you will probably need to install the `postgres` client with your package manager.
Specifically, you will need `libpq`. You can find platform specific instructions by consulting your search engine of choice.

# Setting up the development Postgres server

Install docker, and the main file will handle the rest.

# Setting up the Postgres client

If you're on Fedora, you'll need to symlink the library.
You'll have to run
```bash
$ sudo ln -s /usr/lib64/libpq.so.5 /usr/lib/libpq.so
```

# Setting the environment variables

Tell `diesel` where the database is:

```bash
$ echo DATABASE_URL=postgres://postgres:docker@127.0.0.1:5432 > .env
```

# Creating the tables

Run this to have diesel create the tables:

```bash
$ diesel migration run
```
