# Squares RNG

Implementation of the [Squares CBRNG](https://arxiv.org/abs/2004.06278)

- Counter-based RNGs are non-serial: you can jump to any index in the RNG sequence. This is especially useful in parallel / distributed contexts

- Squares is the fastest known CBRNG, and has higher quality than [Philox](https://www.semanticscholar.org/paper/Parallel-random-numbers%3A-As-easy-as-1%2C-2%2C-3-Salmon-Moraes/7a1bc9d13c484610133ee50ac3126f5adc5b29b9?utm_source=direct_link) at 2x speed

- Squares is still slower than serial RNGs

- Provides `2^64` outputs per key

- This crate is `no_std`, and all functions are `const`

# Example

CBRNGs are stateless. A `key` is like a `seed`.

```rust , ignore
let r32 = squares::u32(key, idx);
let r64 = squares::u64(key, idx);
```

# Admissible Keys

Squares keys are not arbitrary! Many bit patterns can lead to poor output quality.

The `key` function makes an admissible Squares key from a "seed" index, which has no restrictions.

```rust
let key = squares::key(239482304);
```

For a manual key, see `Key::checked` and `Key::unchecked`.

# `rand` Compatibility

Enable the `rand` feature to expose `Squares`, an RNG struct compatible with the `rand` crates.

# Approximate Throughput

Results will vary. On my laptop (M1 Max):

| fn     | time     | per core    |
|--------|----------|-------------|
| `u32`  | `1.29ns` | `3.10 GB/s` |
| `u64`  | `1.65ns` | `4.85 GB/s` |
| `key`  | `24.5ns` | `40.8 M/s`  |

# License

`MIT OR Apache-2.0`