.DEFAULT_GOAL :=migrate 

install:
		cargo install sqlx-cli

create_database: install
		sqlx database create

migrate: create_database
		sqlx migrate run
