include .env
export

COMPOSE_FILE = docker-compose.yml

bd_init:
	sqlx migrate add init_schema

bd_migrate:
	sqlx migrate run

bd_reset:
	sqlx database reset

dev:
	@echo "🔥 Démarrage de l'environnement de développement..."
	docker compose -f $(COMPOSE_FILE) up -d
	@cargo watch -x check -x run
	@echo "✅ Environnement prêt !"