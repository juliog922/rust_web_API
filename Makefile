build:
	@ docker compose build

up:
	@ docker compose up -d

down:
	@ docker compose down

migration:
	@ docker compose exec app diesel migration run

redo:
	@ docker compose exec app diesel migration redo

launch:
	@ docker compose exec app cargo run

test:
	@ docker compose exec app cargo test
	
ping:
	@ docker compose exec postgres psql -U postgres -d app_db -c "SELECT * FROM rustaceans;"
	@ docker compose exec postgres psql -U postgres -d app_db -c "SELECT * FROM crates;"