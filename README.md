# Individual Project

This project is a Rust CLI application that interacts with an SQLite database to perform CRUD operations on car data.

## Features

- **Data Extraction**: Extracts car data from a given CSV URL and saves it locally.
- **Data Loading**: Transforms and loads the extracted data into an SQLite database.
- **Data Querying**: Queries and displays the top 5 rows of the loaded data from the SQLite database.
- **Update Price**: Updates the price of a specific car brand in the SQLite database.

## How to Run

1. **Data Extraction**: `cargo run extract`
2. **Data Loading**: `cargo run load`
3. **Data Querying**: `cargo run query`
4. **Update Price**: `cargo run update_price [brand_name] [new_price]`

For example, to update the price of the brand "Toyota" to 20000, run:

cargo run update_price Toyota 20000

## Dependencies

- `rusqlite`: For interacting with SQLite databases.
- `reqwest`: For making HTTP requests.

To install the dependencies, run:

cargo install rusqlite reqwest

## Use of GitHub Copilot

During the development of this project, GitHub Copilot was an invaluable tool. It provided real-time code suggestions, helping to streamline the coding process and ensure best practices were followed.

## GitHub Actions

The project uses GitHub Actions for continuous integration and continuous deployment (CI/CD). The workflow includes steps for:

- Code formatting
- Code linting
- Running tests
- Building the Rust binary

## Demo Video

A demo video showcasing the functionality of the project can be found [here](#).

## References

- [Rust MLOps Template](https://github.com/nogibjj/rust-mlops-template/tree/main)
- [VSCode Dev Containers](https://github.com/microsoft/vscode-dev-containers/tree/v0.245.2)





