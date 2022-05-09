# Raise an error

```Rust
fn exit(x: i32) {
    if x == 0 {
        panic!("we got a 0");
    }
    println!("things are fine")
}
```

There is a difference between `enum Result<T, E>` and `enum Option<T>`

```Rust
enum Result<T,E>{
    Ok(T),
    Err(E)
}
```

```Rust
enum Option<T>{
    Some(T),
    None
}
```

As we can see both can help handling errors, lets talk about the differences

```Rust
let f: File = match f {
    Ok(file: File) => file,
    Err(error: Error) => panic!("Fail {:?}", error),
};
let f: File = match f {
    Some(file: File) => file,
    None => panic!("Fail with a unknown error"),
};
```

A most sofysticated example

```Rust
let f: File = match f {
    Ok(file: File) => file,
    Err(error: Error) => match error.kind() {
        ErrorKind::NotFound => match File::create("filename.txt") {
            Ok(fc: File) => fc,
            Err(e: Error) => panic!("Error creating file {:?}", e)
        },
        otterError: ErrorKind => {
            panic!("Error {:?}", otherError)
        }

        
        },
};
```

What about a non versobse sollution

```Rust
let f = File::open("filename.txt").wnrap();
let f = File::open("filename.txt").expect("Couldn't open file");
let f = File::open("filename.txt")?
```
