.DEFAULT_GOAL := run

create_database:
		sqlx database create

migrate: create_database
		sqlx migrate run

run: migrate
		cargo run
