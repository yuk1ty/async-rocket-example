# async-rocket-example

Try out repository of Rocket v0.5.0 🎉 Congrats for releasing the version. I tried to launch my first Rocket project on this repo.

## Spin up your environment

Specifically, you need to set up Postgres to launch this example app. Firstly, run the following shell:

```
./postgres.sh
```

Then you can confirm a container runs Postgres. After that, to log in to the container, hit the following command:

```
docker exec -it async-rocket-postgres /bin/bash
```

Run `psql` command after you confirmed that you can successfully log in the container:

```
psql -h localhost -U user
```

## Set up .cargo/config.toml

To compile this repo, you need to set up `.cargo/config.toml` on your local project.

Add the following configuration to the file:

```
# config.toml
[env]
DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres"
```

This is since this repo is using the `macros` feature on sqlx.
