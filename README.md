# async-rocket-example

Try out repository of Rocket v0.5.0 🎉 Congrats for releasing the version.

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
