# pwgen-command

pwgen is a command-line password generator written in Rust.

## Features

*   **Secure by Default**: Generates strong passwords by ensuring at least one character from each selected character set.
*   **Flexible Character Sets**: Easily include lowercase letters, uppercase letters, digits, and symbols.
*   **Custom Characters**: Provide your own set of characters for password generation.
*   **Configurable Length & Count**: Specify the exact length and number of passwords to generate.
*   **File Output**: Print passwords to standard output or save them directly to a file.
*   **User-Friendly**: If no character sets are specified, it defaults to using all of them (`--all`).
*   **Robust Validation**: Provides clear error messages for invalid inputs (e.g., password length too short, invalid custom characters).

## Installation

### Using Cargo

If you have the Rust toolchain installed, you can install `pwgen` directly from the source:

```sh
cargo install --path .
```

###  Building from Source

1.  Clone the repository: `git clone <repository-url>`
2.  Navigate to the project directory: `cd pwgen`
3.  Build the project: `cargo build --release`

The executable will be located at `target/release/pwgen`.

## Usage

```
A command-line password generator.

Usage: pwgen [OPTIONS]

Options:
  -a, --all
          Include all default character categories:
          lowercase, uppercase, digits, and symbols.

  -c, --chars <CHARS>
          Specify an additional set of characters to include in the password.

  -C, --count <COUNT>
          The number of passwords to generate.
          [default: 1]

  -L, --length <LENGTH>
          The total length of the password to be generated.
          [default: 8]

  -l, --use-lower
          Include lowercase letters (a-z) in the password.

  -u, --use-upper
          Include uppercase letters (A-Z) in the password.

  -d, --use-digits
          Include digits (0-9) in the password.

  -s, --use-symbols
          Include symbols or special characters (e.g., !@#) in the password.

  -o, --output <FILE>
          The output file path. If not specified, output to stdout.

  -h, --help
          Print help

  -V, --version
          Print version
```

### Examples

1.  **Generate a default password**
    (8 characters long, using all character sets).

    ```sh
    pwgen
    ```

2.  **Generate a 16-character long password**.

    ```sh
    pwgen -L 16
    ```

3.  **Generate 5 passwords**, each 12 characters long.

    ```sh
    pwgen -L 12 -C 5
    ```

4.  **Generate a password with only lowercase letters and digits**.

    ```sh
    pwgen -L 10 -l -d
    ```

5.  **Generate a password using a custom character set**.

    ```sh
    pwgen -L 8 -c "abcdef123456"
    ```

6.  **Generate 10 passwords and save them to a file**.

    ```sh
    pwgen -L 20 -C 10 -o passwords.txt
    ```

## License

*   MIT license
