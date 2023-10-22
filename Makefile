# Rust targets
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			# Rust compiler
	cargo --version 			# Rust package manager
	rustfmt --version			# Rust code formatter
	rustup --version			# Rust toolchain manager
	clippy-driver --version		# Rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

create:
	cargo run query "INSERT INTO CarsDB (Brand, Model, Year, Price) VALUES ('Toyota', 'Corolla', 2022, 20000);"

read:
	cargo run query "SELECT * FROM CarsDB WHERE Brand = 'Toyota';"

update:
	cargo run query "UPDATE CarsDB SET Model='Camry', Year=2023, Price=25000 WHERE Brand = 'Toyota';"

delete:
	cargo run query "DELETE FROM CarsDB WHERE Brand = 'Toyota';"

release:
	cargo build --release

clean:
	cargo clean

docs:
	cargo doc --open

update:
	rustup update
	cargo update

check:
	cargo check

# Combined targets for convenience
all: format lint test run

generate_and_push:
	# Add, commit, and push the generated files to GitHub
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add metric log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
