include .env
export

bd_init:
	sqlx migrate add init_schema

bd_migrate:
	sqlx migrate run

bd_reset:
	sqlx database reset

dev:
	@echo "Starting development server..."
	@cargo watch -x check -x run