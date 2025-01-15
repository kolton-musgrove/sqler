# SQLer

An opinionated SQL formatter for consistent, read-at-a-glance SQL.

![sqler-sql-formatter-pig-nose](https://github.com/user-attachments/assets/db39fa47-fd4d-4e76-8008-ed1ee329a422)

## Features

- Formats SQL queries with customizable indentation
- Supports common SQL elements:
  - SELECT
  - WHERE
  - GROUP BY
  - aliasing
  - table and schema referencing
  - binary ops
- Feature-flag based SQL dialect delineation!

## Installation

Add sqler to your project's deps:

```toml
sqler = "0.1.0"
```

## Usage

```rust
use sqler::{Config, format_sql};

let sql = "SELECT id, name AS full_name FROM users WHERE age > 18";

// use default configuration
let config = Config::default();

match format_sql(sql, &config) {
  Ok(formatted) => println!("{}", formatted),
  Err(e) => eprintln!("Error: {}", e),
}
```

### Configuration

You can customize the following formatting behavior:

```rust
use sqler::Config;

let config = Config {
  indent_char: " ".to_string(),
  indent_width: 2,
  max_line_length: 120,
};
```

## Building from Source

1. Clone the repo
2. Build the project:

```bash
cargo build --release
```
