f32 和 f64 浮点数，没有实现 `std::cmp::Eq` 特征，因此不可以用作 HashMap 的 Key。

目前，HashMap 使用的哈希函数是 SipHash，它的性能不是很高，但是安全性很高。SipHash 在中等大小的 Key 上，性能相当不错，但是对于小型的 Key （例如整数）或者大型 Key （例如字符串）来说，性能还是不够好。若你需要极致性能，例如实现算法，可以考虑这个库：[ahash](https://github.com/tkaitchuck/ahash)。
