```rust
for region in regions.iter() {
    println!("{}", &region);
}
```

`println!("{}", &region);` 

region 前面的 `&` 似乎无论有多少个（0-N），都能正常输出。