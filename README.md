# make_public

`make_public` is a procedural macro attribute for Rust that automatically makes all fields in a struct public.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
make_public = "0.1.0"
```

## Usage

```rust
use make_public::make_public;

// All fields will be public
#[make_public]
struct MyStruct {
    field1: String,
    field2: i32,
    field3: bool,
}

fn main() {
    let my_struct = MyStruct {
        field1: String::from("Hello"),
        field2: 42,
        field3: true,
    };
    
    // Fields are accessible from outside
    println!("{}", my_struct.field1);
}
```

## Features

- Makes all fields in a struct public
- Works with any field type
- Preserves other attributes (like `derive`)
- Simple and easy to use

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
