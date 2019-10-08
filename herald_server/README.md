This file will walk you through all the steps necessary to get the server running.

# Prerequisites

- `cargo`, at least version 1.39
- `docker`, `docker-compose`
- `libpq`

# I'm impatient and just want to paste into my terminal

Well alright then.
The first time you run the server, you'll want to run these commands:

```bash
cargo install diesel_cli --no-default-features --features postgres &\
echo DATABASE_URL=postgres://postgres:docker@127.0.0.1:5432 > .env &\
docker-compose up -d &\
diesel migration run &\
docker-compose down
```

Then you can run the server with these commands:

```bash
docker-compose up -d &\
cargo run --release
```

# Setting up diesel

Install the `diesel_cli`:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

You'll also need to tell `diesel` where the database is.
You can do this by adding a line setting `DATABASE_URL` to `.env`.
If you're using the local dev server (you probably are), this is the command to run:

```bash
echo DATABASE_URL=postgres://postgres:docker@127.0.0.1:5432 > .env
```

# Setting up the development Postgres server

Once you've installed `diesel`, you'll want to set up tables for postgres.
To start the server, you'll want to run

Once you've done that, you can start the postgres instance by running 

```bash
docker-compose up -d
```

and you can stop it with

```bash
docker compose down
```

Additionally, if you want to reset the server, you can run `pg_clean.sh`, which will shut down the container and remove the volume.

# Creating the tables

While the database is online, you can run this command to have `diesel` create the tables:

```bash
$ diesel migration run
```

# Running the HTTP server

Once you've done all the above, you're ready to run the server.
First, start the postgres instance, then call `cargo run`, or `cargo run --release` if you want a faster server and a slower compiler.

# Dealing with linker issues

On newer versions of Fedora, `ld` sometimes has issues finding `libpq`. 
You can solve this by symlinking the `.so` file from `/usr/lib64` to `/usr/lib`. 
If you run this command and it still doesn't work, something else is going wrong:

```bash
$ sudo ln -s /usr/lib64/libpq.so.5 /usr/lib/libpq.so
```

