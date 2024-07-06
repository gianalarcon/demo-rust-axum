# Demo Rust Axum API

This is a simple demo project showcasing how to build APIs using Rust and Axum framework.

## Installation

0. Install Rust: <https://www.rust-lang.org/tools/install>
1. Clone the repository: `git clone https://github.com/gianalarcon/demo-rust-axum.git`
2. Navigate to the project directory: `cd demo-rust-axum`
3. Install dependencies: `cargo build`

## Usage

Copy the `.env.example` file to `.env` and set the value of the `TEXT` environment variable.

```bash
cp .env.example .env
```

To start the server, run the following command:

```bash
cargo run
```

The server will start running on `http://localhost:3000`.

## API Endpoints

- `GET /read-env`: Reads the value of the `TEXT` environment variable.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
