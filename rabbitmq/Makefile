start-rabbitmq:
	docker-compose up -d

stop-rabbitmq:
	docker-compose stop

dev:
	cargo run

clean:
	cargo clean

style-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings
