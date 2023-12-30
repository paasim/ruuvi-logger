include .env
export

.PHONY: init-db migrate

init-db: records.db
	make migrate
	cargo sqlx prepare

migrate:
	sqlx migrate run

records.db:
	sqlx database create
