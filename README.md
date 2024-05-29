## Preparation
- docker
- rust

## install
```
git clone https://github.com/bedrinth/labrinth
docker-compose up -d
cargo install --git https://github.com/launchbadge/sqlx sqlx-cli --no-default-features --features postgres,rustls
sqlx database setup
cargo run
```

## migrant data volume
```
docker run --volumes-from db-data -v ${pwd}:/backup ubuntu tar cvf /backup/backup.tar /var/lib/postgresql/data
sudo docker run -v db-data:/var/lib/postgresql/data --name tmp ubuntu /bin/bash
docker run --volumes-from db-data -v $(pwd):/backup busybox tar xvf /backup/backup.tar /var/lib/postgresql/data
```