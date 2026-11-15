

dev:	
	cargo watch -q -c -x run
test:	
	cargo watch -x test
build:	
	cargo build --release

db-generate: 
	sea-orm-cli generate entity -o ./src/entities --entity-format dense

db-migrate:
	sea-orm-cli migrate up

db-migrate-down:
	sea-orm-cli migrate down