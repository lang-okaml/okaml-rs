# okaml-rs

okaml-rs is a Rust library for parsing [okaml](github.com/lang-okaml).

## Installation

Use the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install foobar.

```bash
cargo add okaml
```

## Usage

```rust
use okaml;

fn main() {
    let file_path = "../example/syntax.okml";
    let parsed_tree:Vec<Okml> = Okml::read_from_file(file_path);
    println!("{:?}", parsed_ast);
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
