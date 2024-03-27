# tipjar

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
