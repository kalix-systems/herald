# Setting up diesel

Install the `diesel_cli`:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Note: you will probably need to install the `postgres` client with your package manager.
Specifically, you will need `libpq`. You can find platform specific instructions by consulting your search engine of choice.

# Setting up the development Postgres server

Install the docker image:

```bash
$ docker pull postgres
```

Start the container by running this from the `server` root directory:

```bash
$ docker run --rm   --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v $PWD/postgres_volume:/var/lib/postgresql/data  postgres
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
