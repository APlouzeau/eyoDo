include .env
export

COMPOSE_FILE = docker-compose.yml

bd_init:
	sqlx migrate add init_schema

bd_migrate:
	sqlx migrate run

bd_reset:
	docker exec -it eyodo-database-dev psql -U postgres -d postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'eyodo-db' AND pid <> pg_backend_pid();"
	sqlx database reset

dev:
	@echo "🔥 Démarrage de l'environnement de développement..."
	docker compose -f $(COMPOSE_FILE) up -d
	@cargo watch -x check -x run
	@echo "✅ Environnement prêt !"