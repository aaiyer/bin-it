# Bin-It

**Bin-It** is a simple, efficient Rust library for binary serialization and deserialization. With a focus on 
performance and ease of use, Bin-It lets you seamlessly serialize Rust types into compact binary formats and 
read them back with precision. Whether you're storing data in binary files, transmitting data over networks, 
or handling low-level byte operations, **Bin-It** has you covered.

## Features

- Serialize and deserialize common primitive types (`u8`, `i16`, `f32`, etc.).
- Supports serialization of strings and collections (e.g., `Vec<u8>`, `Vec<f64>`, etc.).
- Consistent, little-endian encoding for cross-platform compatibility.
- Minimal dependencies for fast, lightweight binary manipulation.

## Usage

### Writing Data

The BinaryWriter struct allows you to serialize various data types into a binary buffer:

```rust
use bin_it::BinaryWriter;

fn main() {
    let mut writer = BinaryWriter::new();
    writer.write_u32(42);
    writer.write_string("Hello, Bin-It!");
    writer.write_f64(3.14159);
    
    let data = writer.get_data();
    // Now `data` contains the binary representation of the serialized values.
}
```


### Reading Data

The BinaryReader struct lets you deserialize the binary data back into Rust types:

```rust
use bin_it::BinaryReader;

fn main() {
    let data = vec![42, /* binary data goes here */];
    let mut reader = BinaryReader::new(&data);

    let number = reader.read_u32().unwrap();
    let text = reader.read_string().unwrap();
    let pi = reader.read_f64().unwrap();

    println!("Number: {}", number);
    println!("Text: {}", text);
    println!("Pi: {}", pi);
}
```

## Supported Data Types

**Bin-It** supports writing and reading of:
 * Primitives: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, and bool.
 * Strings: UTF-8 strings serialized with length-prefix encoding.
 * Collections: Fixed-size collections, such as Vec<T> for supported types.

