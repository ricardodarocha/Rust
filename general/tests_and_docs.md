In this section I will do an overview of 2 kind of relevant rust code structures  
They are code documentation and code tests 

to give your code documentaion superpower just write comments with 3 /// instead of 2 // 

/// will appear as discussion text at generated documentaion

lets create an example
```Rust
/// this method provides a very good aproximation of pi using the razor 22/7
fn rpi {22/7}
/// # Examples
/// ```
///fn circleArea (r) {
///    
///    r * r * rpi()
///}
/// ```
```
You can also write testable sentence inside those comment strucutures  
Use **cargo rustdoc --test** to try this

```Rust
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// let a = circleArea(1);
/// assert!(a, rpi())
/// ```
```
