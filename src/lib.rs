/// # Bin-It
///
/// **Bin-It** is a simple, efficient Rust library for binary serialization and deserialization. With a focus on
/// performance and ease of use, Bin-It lets you seamlessly serialize Rust types into compact binary formats and
/// read them back with precision. Whether you're storing data in binary files, transmitting data over networks,
/// or handling low-level byte operations, **Bin-It** has you covered.
///
/// ## Features
///
/// - Serialize and deserialize common primitive types (`u8`, `i16`, `f32`, etc.).
/// - Supports serialization of strings and collections (e.g., `Vec<u8>`, `Vec<f64>`, etc.).
/// - Consistent, little-endian encoding for cross-platform compatibility.
/// - Minimal dependencies for fast, lightweight binary manipulation.
///
/// ## Usage
///
/// ### Writing Data
///
/// The BinaryWriter struct allows you to serialize various data types into a binary buffer:
///
/// ```rust
/// use bin_it::BinaryWriter;
///
/// fn main() {
///     let mut writer = BinaryWriter::new();
///     writer.write_u32(42);
///     writer.write_string("Hello, Bin-It!");
///     writer.write_f64(3.14159);
///
///     let data = writer.get_data();
///     // Now `data` contains the binary representation of the serialized values.
/// }
/// ```
///
///
/// ### Reading Data
///
/// The BinaryReader struct lets you deserialize the binary data back into Rust types:
///
/// ```rust
/// use bin_it::BinaryReader;
///
/// fn main() {
///     // Ensure `data` has enough bytes for the expected reads
///     let data = vec![42, 0, 0, 0]; // Sufficient data for a u32
///     let mut reader = BinaryReader::new(&data);
///
///     match reader.read_u32() {
///         Ok(number) => println!("Number: {}", number),
///         Err(e) => println!("Error reading u32: {}", e),
///     }
/// }
/// ```
///
/// ## Supported Data Types
///
/// **Bin-It** supports writing and reading of:
///  * Primitives: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, and bool.
///  * Strings: UTF-8 strings serialized with length-prefix encoding.
///  * Collections: Fixed-size collections, such as Vec<T> for supported types.

use std::convert::TryInto;


/// BinaryWriter is used to serialize various data types into a byte buffer.
pub struct BinaryWriter {
  data: Vec<u8>,
}

impl BinaryWriter {
  /// Creates a new BinaryWriter with an empty buffer.
  pub fn new() -> Self {
    BinaryWriter { data: Vec::new() }
  }

  /// Returns a reference to the internal byte buffer.
  pub fn get_data(self) -> Vec<u8> {
    self.data
  }

  /// Writes a u8 value to the buffer.
  pub fn write_u8(&mut self, value: u8) {
    self.data.push(value);
  }

  /// Writes a u16 value to the buffer in little-endian order.
  pub fn write_u16(&mut self, value: u16) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes a u32 value to the buffer in little-endian order.
  pub fn write_u32(&mut self, value: u32) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes a u64 value to the buffer in little-endian order.
  pub fn write_u64(&mut self, value: u64) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes an i8 value to the buffer.
  pub fn write_i8(&mut self, value: i8) {
    self.data.push(value as u8);
  }

  /// Writes an i16 value to the buffer in little-endian order.
  pub fn write_i16(&mut self, value: i16) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes an i32 value to the buffer in little-endian order.
  pub fn write_i32(&mut self, value: i32) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes an i64 value to the buffer in little-endian order.
  pub fn write_i64(&mut self, value: i64) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes a f32 value to the buffer in little-endian order.
  pub fn write_f32(&mut self, value: f32) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes a f64 value to the buffer in little-endian order.
  pub fn write_f64(&mut self, value: f64) {
    self.data.extend(&value.to_le_bytes());
  }

  /// Writes a bool value to the buffer as a single byte (0 or 1).
  pub fn write_bool(&mut self, value: bool) {
    self.data.push(if value { 1 } else { 0 });
  }

  /// Writes a string to the buffer. First writes the length as u32, then the UTF-8 bytes.
  pub fn write_string(&mut self, value: &str) {
    let bytes = value.as_bytes();
    self.write_u32(bytes.len() as u32);
    self.data.extend(bytes);
  }

  /// Writes a vector of u8 to the buffer. First writes the length as u32, then the bytes.
  pub fn write_vec_u8(&mut self, value: &[u8]) {
    self.write_u32(value.len() as u32);
    self.data.extend(value);
  }

  /// Writes a vector of u16 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_u16(&mut self, value: &[u16]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_u16(v);
    }
  }

  /// Writes a vector of u32 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_u32(&mut self, value: &[u32]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_u32(v);
    }
  }

  /// Writes a vector of u64 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_u64(&mut self, value: &[u64]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_u64(v);
    }
  }

  /// Writes a vector of i8 to the buffer. First writes the length as u32, then the bytes.
  pub fn write_vec_i8(&mut self, value: &[i8]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_i8(v);
    }
  }

  /// Writes a vector of i16 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_i16(&mut self, value: &[i16]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_i16(v);
    }
  }

  /// Writes a vector of i32 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_i32(&mut self, value: &[i32]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_i32(v);
    }
  }

  /// Writes a vector of i64 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_i64(&mut self, value: &[i64]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_i64(v);
    }
  }

  /// Writes a vector of f32 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_f32(&mut self, value: &[f32]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_f32(v);
    }
  }

  /// Writes a vector of f64 to the buffer. First writes the length as u32, then the bytes in little-endian.
  pub fn write_vec_f64(&mut self, value: &[f64]) {
    self.write_u32(value.len() as u32);
    for &v in value {
      self.write_f64(v);
    }
  }

  /// Writes a vector of strings to the buffer. First writes the length as u32, then each string serialized.
  pub fn write_vec_string(&mut self, value: &[String]) {
    self.write_u32(value.len() as u32);
    for s in value {
      self.write_string(s);
    }
  }
}

/// BinaryReader is used to deserialize various data types from a byte buffer.
pub struct BinaryReader<'a> {
  data: &'a [u8],
  cursor: usize,
}

impl<'a> BinaryReader<'a> {
  /// Creates a new BinaryReader with the given byte slice.
  pub fn new(data: &'a [u8]) -> Self {
    BinaryReader { data, cursor: 0 }
  }

  /// Reads a u8 value from the buffer.
  pub fn read_u8(&mut self) -> Result<u8, String> {
    self.ensure_available(1)?;
    let value = self.data[self.cursor];
    self.cursor += 1;
    Ok(value)
  }

  /// Reads a u16 value from the buffer in little-endian order.
  pub fn read_u16(&mut self) -> Result<u16, String> {
    self.ensure_available(2)?;
    let bytes = &self.data[self.cursor..self.cursor + 2];
    self.cursor += 2;
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
  }

  /// Reads a u32 value from the buffer in little-endian order.
  pub fn read_u32(&mut self) -> Result<u32, String> {
    self.ensure_available(4)?;
    let bytes = &self.data[self.cursor..self.cursor + 4];
    self.cursor += 4;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
  }

  /// Reads a u64 value from the buffer in little-endian order.
  pub fn read_u64(&mut self) -> Result<u64, String> {
    self.ensure_available(8)?;
    let bytes = &self.data[self.cursor..self.cursor + 8];
    self.cursor += 8;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
  }

  /// Reads an i8 value from the buffer.
  pub fn read_i8(&mut self) -> Result<i8, String> {
    self.ensure_available(1)?;
    let value = self.data[self.cursor] as i8;
    self.cursor += 1;
    Ok(value)
  }

  /// Reads an i16 value from the buffer in little-endian order.
  pub fn read_i16(&mut self) -> Result<i16, String> {
    self.read_u16().map(|v| v as i16)
  }

  /// Reads an i32 value from the buffer in little-endian order.
  pub fn read_i32(&mut self) -> Result<i32, String> {
    self.read_u32().map(|v| v as i32)
  }

  /// Reads an i64 value from the buffer in little-endian order.
  pub fn read_i64(&mut self) -> Result<i64, String> {
    self.read_u64().map(|v| v as i64)
  }

  /// Reads a f32 value from the buffer in little-endian order.
  pub fn read_f32(&mut self) -> Result<f32, String> {
    self.ensure_available(4)?;
    let bytes = &self.data[self.cursor..self.cursor + 4];
    self.cursor += 4;
    Ok(f32::from_le_bytes(bytes.try_into().unwrap()))
  }

  /// Reads a f64 value from the buffer in little-endian order.
  pub fn read_f64(&mut self) -> Result<f64, String> {
    self.ensure_available(8)?;
    let bytes = &self.data[self.cursor..self.cursor + 8];
    self.cursor += 8;
    Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
  }

  /// Reads a bool value from the buffer (expects 0 or 1).
  pub fn read_bool(&mut self) -> Result<bool, String> {
    self.read_u8().map(|v| match v {
      0 => false,
      1 => true,
      _ => panic!("Invalid boolean value: {}", v),
    })
  }

  /// Reads a string from the buffer. Expects a u32 length followed by UTF-8 bytes.
  pub fn read_string(&mut self) -> Result<String, String> {
    let length = self.read_u32()? as usize;
    self.ensure_available(length)?;
    let bytes = &self.data[self.cursor..self.cursor + length];
    self.cursor += length;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
  }

  /// Reads a vector of u8 from the buffer. Expects a u32 length followed by bytes.
  pub fn read_vec_u8(&mut self) -> Result<Vec<u8>, String> {
    let length = self.read_u32()? as usize;
    self.ensure_available(length)?;
    let vec = self.data[self.cursor..self.cursor + length].to_vec();
    self.cursor += length;
    Ok(vec)
  }

  /// Reads a vector of u16 from the buffer. Expects a u32 length followed by u16 values.
  pub fn read_vec_u16(&mut self) -> Result<Vec<u16>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_u16()?);
    }
    Ok(vec)
  }

  /// Reads a vector of u32 from the buffer. Expects a u32 length followed by u32 values.
  pub fn read_vec_u32(&mut self) -> Result<Vec<u32>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_u32()?);
    }
    Ok(vec)
  }

  /// Reads a vector of u64 from the buffer. Expects a u32 length followed by u64 values.
  pub fn read_vec_u64(&mut self) -> Result<Vec<u64>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_u64()?);
    }
    Ok(vec)
  }

  /// Reads a vector of i8 from the buffer. Expects a u32 length followed by i8 values.
  pub fn read_vec_i8(&mut self) -> Result<Vec<i8>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_i8()?);
    }
    Ok(vec)
  }

  /// Reads a vector of i16 from the buffer. Expects a u32 length followed by i16 values.
  pub fn read_vec_i16(&mut self) -> Result<Vec<i16>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_i16()?);
    }
    Ok(vec)
  }

  /// Reads a vector of i32 from the buffer. Expects a u32 length followed by i32 values.
  pub fn read_vec_i32(&mut self) -> Result<Vec<i32>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_i32()?);
    }
    Ok(vec)
  }

  /// Reads a vector of i64 from the buffer. Expects a u32 length followed by i64 values.
  pub fn read_vec_i64(&mut self) -> Result<Vec<i64>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_i64()?);
    }
    Ok(vec)
  }

  /// Reads a vector of f32 from the buffer. Expects a u32 length followed by f32 values.
  pub fn read_vec_f32(&mut self) -> Result<Vec<f32>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_f32()?);
    }
    Ok(vec)
  }

  /// Reads a vector of f64 from the buffer. Expects a u32 length followed by f64 values.
  pub fn read_vec_f64(&mut self) -> Result<Vec<f64>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_f64()?);
    }
    Ok(vec)
  }

  /// Reads a vector of strings from the buffer. Expects a u32 length followed by serialized strings.
  pub fn read_vec_string(&mut self) -> Result<Vec<String>, String> {
    let length = self.read_u32()? as usize;
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
      vec.push(self.read_string()?);
    }
    Ok(vec)
  }

  /// Ensures that there are at least `size` bytes available to read.
  fn ensure_available(&self, size: usize) -> Result<(), String> {
    if self.cursor + size > self.data.len() {
      Err("Unexpected end of data".to_string())
    } else {
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_binary_writer_reader() {
    let mut writer = BinaryWriter::new();

    // Write various data types
    writer.write_u8(255);
    writer.write_i8(-128);
    writer.write_u16(65535);
    writer.write_i16(-32768);
    writer.write_u32(4294967295);
    writer.write_i32(-2147483648);
    writer.write_u64(18446744073709551615);
    writer.write_i64(-9223372036854775808);
    writer.write_f32(3.1415927);
    writer.write_f64(2.718281828459045);
    writer.write_bool(true);
    writer.write_string("Hello, World!");

    let data = writer.get_data().clone();

    let mut reader = BinaryReader::new(&data);

    // Read and assert the values
    assert_eq!(reader.read_u8().unwrap(), 255);
    assert_eq!(reader.read_i8().unwrap(), -128);
    assert_eq!(reader.read_u16().unwrap(), 65535);
    assert_eq!(reader.read_i16().unwrap(), -32768);
    assert_eq!(reader.read_u32().unwrap(), 4294967295);
    assert_eq!(reader.read_i32().unwrap(), -2147483648);
    assert_eq!(reader.read_u64().unwrap(), 18446744073709551615);
    assert_eq!(reader.read_i64().unwrap(), -9223372036854775808);
    assert!((reader.read_f32().unwrap() - 3.1415927).abs() < 1e-6);
    assert!((reader.read_f64().unwrap() - 2.718281828459045).abs() < 1e-12);
    assert_eq!(reader.read_bool().unwrap(), true);
    assert_eq!(reader.read_string().unwrap(), "Hello, World!");
  }

  #[test]
  fn test_binary_writer_reader_vectors() {
    let mut writer = BinaryWriter::new();

    // Write vectors
    writer.write_vec_u8(&[1, 2, 3, 4, 5]);
    writer.write_vec_i16(&[-1, -2, -3]);
    writer.write_vec_f64(&[1.1, 2.2, 3.3]);

    let data = writer.get_data().clone();

    let mut reader = BinaryReader::new(&data);

    // Read and assert the vectors
    assert_eq!(reader.read_vec_u8().unwrap(), vec![1, 2, 3, 4, 5]);
    assert_eq!(reader.read_vec_i16().unwrap(), vec![-1, -2, -3]);
    let read_f64 = reader.read_vec_f64().unwrap();
    assert_eq!(read_f64.len(), 3);
    assert!((read_f64[0] - 1.1).abs() < 1e-10);
    assert!((read_f64[1] - 2.2).abs() < 1e-10);
    assert!((read_f64[2] - 3.3).abs() < 1e-10);
  }

  #[test]
  fn test_binary_writer_reader_vec_string() {
    let mut writer = BinaryWriter::new();

    // Write a vector of strings
    let strings = vec![
      "Hello".to_string(),
      "Bin-It".to_string(),
      "Serialization".to_string(),
      "".to_string(),
      "🚀✨".to_string(),
    ];
    writer.write_vec_string(&strings);

    let data = writer.get_data().clone();

    let mut reader = BinaryReader::new(&data);

    // Read and assert the vector of strings
    let read_strings = reader.read_vec_string().unwrap();
    assert_eq!(read_strings, strings);
  }

  #[test]
  fn test_binary_reader_error() {
    let data = vec![1, 2]; // Insufficient data for a u32

    let mut reader = BinaryReader::new(&data);

    // Attempt to read a u32, which should fail
    assert!(reader.read_u32().is_err());
  }
}
