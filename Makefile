install:
	@echo "Checking if Rust is installed..."
	@which rustc &> /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	@echo "Installing project dependencies..."
	@cargo build


# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Extract data
extract: 
	cargo run extract

# Transform and Load data
transform_load:
	cargo run transform_load

# Query the top 5 rows from the CarsDB table
query:
	cargo run query

create-table:
    cargo run query "CREATE TABLE IF NOT EXISTS CarsDB (Car TEXT, MPG REAL, Cylinders INTEGER, Displacement REAL, Horsepower REAL, Weight REAL, Acceleration REAL, Model INTEGER, Origin TEXT);"

create:
	cargo run query "INSERT INTO CarsDB (Car, MPG, Cylinders, Displacement, Horsepower, Weight, Acceleration, Model, Origin) VALUES ('Example Car', 25.0, 4, 150.0, 100.0, 2500.0, 10.0, 2023, 'US');"

# Read a specific car entry from the CarsDB table using the Car name
read:
	cargo run query "SELECT * FROM CarsDB WHERE Car = 'Chevrolet Chevelle Malibu';"

# Update a specific car entry in the CarsDB table
update:
	cargo run query "UPDATE CarsDB SET MPG=20.0, Cylinders=6 WHERE Car = 'Chevrolet Chevelle Malibu';"

# Delete a specific car entry from the CarsDB table
delete:
	cargo run query "DELETE FROM CarsDB WHERE Car = 'Chevrolet Chevelle Malibu';"

# Run all formatting, linting, and testing tasks
all: format lint test run

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local usetest.rsr.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
