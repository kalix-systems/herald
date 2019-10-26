This file will walk you through all the steps necessary to get the server running.

# Prerequisites

- `cargo`, at least version 1.39
- `docker`, `docker-compose`

# I'm impatient and just want to paste into my terminal

Well alright then.
The first time you run the server, you'll want to run these commands:

```bash
docker-compose up -d &\
cd server_store &\
cargo run --bin setup_db &\ 
cd .. &\
docker-compose down
```

Then you can run the server with these commands:

```bash
docker-compose up -d &\
cargo run --release
```

# Setting up the development Postgres server


You can start the postgres instance by running 

```bash
docker-compose up -d
```

Now you'll want to set up tables for postgres:

```bash
cargo run --bin setup_db
```

and you can stop the postgres server with

```bash
docker compose down
```

# Resetting the database

To reset all tables in the database to their original condition, deleting all data
in the process, run:

```bash
cargo run --bin reset_db
```

# Running the HTTP server

Once you've done all the above, you're ready to run the server.
First, start the postgres instance, then call `cargo run`, or `cargo run --release` if you want a faster server and a slower compiler.
