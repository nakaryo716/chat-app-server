.DEFAULT_GOAL := run

create_database:
		sqlx database create

migrate: create_database
		sqlx migrate run

release_build: migrate
		cargo build --release

run: release_build
		./target/release/server
