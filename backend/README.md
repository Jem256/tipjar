# tipjar

## Env File Contents
DATABASE_URL=postgres://<POSTGRES_user>:<POSTGRES_db password>@localhost/<POSTGRES_db_name>

The values below are obtained from your polar node
ADDRESS=https://{GRPC Host}
CERT_FILE_PATH=TLS Cert
MACAROON_FILE_PATH=Admin Macaroon

## Docker Set Up

Docker command: 
Run the command below to setup the docker environment for tipjar.
```
docker compose up -d

docker exec -ti db /bin/bash

docker-compose run tipjar
```
Run the command below to add migration
```
diesel setup
```
