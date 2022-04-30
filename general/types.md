# Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You’ll likely recognize these from other programming languages, but let’s jump into how they work in Rust.

## Integer Types
An integer is a number without a fractional component. We used one integer type earlier in this chapter, the u32 type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i instead of u) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. Each variant in the Signed and Unsigned columns (for example, i16) can be used to declare the type of an integer value.

## Integer

Table: Integer Types in Rust
| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

The integer Byte Range

| DATA | TYPE        | MIN MAX    | e      |
|------|-------------|------------|--------|
| i8   | -128        | 127        |        |
| u8   | 0           | 255        |        |
| i16  | -32768      | 32767      | 6x10e4 |
| u16  | -32768      | 65535      | 6x10e4 |
| i32  | -2147483648 | 2147483647 | 2x10e9 |
| u32  | -2147483648 | 4294967295 | 4x10e9 |
                                                              1000000000
special cases 
|      |                                          |                                         |         |
|------|------------------------------------------|-----------------------------------------|---------|
| i64  | -9223372036854775808                     | 9223372036854775807                     | 9x10e18 |
| u64  | 0                                        | 18446744073709551615                    | 1x10e19 |
| i128 | -170141183460469231731687303715884105728 | 170141183460469231731687303715884105727 | 1x10e38 |
| u128 | 0                                        | 340282366920938463463374607431768211455 | 3x10e38 |


## floating-point numbers

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

Here’s an example that shows floating-point numbers in action:

``` Rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    let y: f64 = 3.0; // f64
}
```

Look the study for ABNT - Brasil especification to round Float Number []

Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

## Booleans

Boleans are defined by keyword **bool**

A standard boolean. Can be either `true` or `false`

```Rust
let t = true;
let f = false;

```

## characters

char
A 4 byte character.

```Rust
let a = 'a';
let b = 'b';
let keyboard = '⌨';
```

For example, remove a list of emojy of a string

``` Rust
/// let demojified_string = demoji(String::from("⚡hel✅🙂lo🙂"))
pub fn demoji(string: String) -> String {
    let regex = Regex::new(concat!(
        "[",
        "\u{01F600}-\u{01F64F}", // emoticons
        "\u{01F300}-\u{01F5FF}", // symbols & pictographs
        "\u{01F680}-\u{01F6FF}", // transport & map symbols
        "\u{01F1E0}-\u{01F1FF}", // flags (iOS)
        "\u{002702}-\u{0027B0}",
        "\u{0024C2}-\u{01F251}",
        "]+",
    ))
    .unwrap();

    regex.replace_all(&string, "").to_string()
}
```

