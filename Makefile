clean-deps:
	docker compose down

start-deps:
	docker compose up -d 

reset-deps: clean-deps start-deps

