[![Rust CI/CD](https://github.com/nogibjj/Individual_Project_Yabei/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Individual_Project_Yabei/actions/workflows/cicd.yml)
# Individual Project

This project is a command-line interface (CLI) application written in Rust that utilizes an SQLite database for storing and querying car data. The application can extract data from a CSV file, transform the data into the desired format, and then load it into an SQLite database.

## Explanation of the project
This CLI tool allows for:

- Extracting car data from a remote CSV file.
- Transforming the data and loading it into an SQLite database.
- Running SQL queries directly from the command-line on the loaded data.

It also logs all SQL queries executed against the database for tracking and auditing purposes.

## Features

- **Data Extraction**: The `extract` function is responsible for fetching car data from a specified URL that points to a CSV file. Once the data is fetched, it saves this CSV data to a local file, allowing for subsequent processing without needing to re-fetch the data from the internet.

- **Data Transformation and Loading**: The `transform_load` function takes the locally saved CSV file, reads its contents, and then transforms this data into a format suitable for the SQLite database. After the transformation, the function loads (or inserts) the data into the SQLite database. This ensures the raw data from the CSV is stored in a structured and queryable format.

- **Data Querying**: The `query` function provides the capability to execute SQL queries directly against the SQLite database. This function supports various SQL operations, including SELECT, INSERT, UPDATE, and DELETE. The results of SELECT queries, like fetching the top 5 rows from the database, are displayed directly in the command line, allowing users to immediately view and verify the data.
## How to Run

1. **Data Extraction**:
   ```bash
   cargo run extract
   ```
   or simply
   ```bash
   make extract
   ```
3. **Data Loading**:
   ```bash
   cargo run transform_load
   ```
   or simply
   ```bash
   make transform_load
   ```
5. **Data Querying**:
   ```bash
   cargo run query
   ```
   or simply
   ```bash
   make query
   ```

## Dependencies

- `rusqlite`: For interacting with SQLite databases.
- `reqwest`: For making HTTP requests.
- `csv`: For parsing CSV data

To install the dependencies, run:

```bash
cargo build
```

## Utilizing GitHub Copilot

GitHub Copilot is an AI-powered code completion tool developed by GitHub in collaboration with OpenAI. Throughout the development of this project, GitHub Copilot served as an invaluable assistant, offering the following benefits:

### Code Suggestions
- **Instant Recommendations**: As I typed, Copilot provided real-time code suggestions, helping speed up the development process.
- **Contextual Understanding**: Copilot understood the context of the code, offering relevant method and variable name suggestions.

### Error Handling
- **Identifying Mistakes**: Copilot highlighted potential errors in the code, suggesting more optimal or safer alternatives.
- **Exception Handling**: It provided guidance on handling potential exceptions and edge cases in the code.

### Documentation and Comments
- **Auto-generating Comments**: Copilot assisted in generating meaningful comments for functions and complex code blocks, enhancing code readability.
- **API References**: When working with unfamiliar libraries or APIs, Copilot provided quick references and usage examples.

### Learning and Exploration
- **Exploring New Libraries**: Copilot suggested best practices and usage patterns when integrating new Rust libraries.
- **Deepening Rust Knowledge**: By suggesting idiomatic Rust code, Copilot helped reinforce and deepen my understanding of Rust's unique features.

In summary, GitHub Copilot was not just a tool but a collaborative partner in this project. It streamlined the coding process, reduced the likelihood of errors, and provided continuous learning opportunities.


## GitHub Actions

The project uses GitHub Actions for continuous integration and continuous deployment (CI/CD). The workflow includes steps for:

- Code formatting
- Code linting
- Running tests
- Building the Rust binary
  - for the binary download: go to `Git Actions`, clicking the `Workflows`, and you can find the download link at the bottom of the page

## Demo Video

A demo video showcasing the functionality of the project can be found [here](#).

## References

- [Rust MLOps Template](https://github.com/nogibjj/rust-mlops-template/tree/main)
- [VSCode Dev Containers](https://github.com/microsoft/vscode-dev-containers/tree/v0.245.2)





