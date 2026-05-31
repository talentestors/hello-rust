## 用 Fn 特征解决闭包生命周期

```rust
fn main() {
   let closure_slision = fun(|x: &i32| -> &i32 { x });
   assert_eq!(*closure_slision(&45), 45);
   // Passed !
}

fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
   f
}
```
