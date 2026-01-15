# fast-des 🚀

This crate implements DES using a bitsliced implementation. I set out to teach myself both the inner workings of DES and [bitslicing](https://timtaubert.de/blog/2018/08/bitslicing-an-introduction/). The bitsliced implementation achieved a 10x in performance.

## Features

This crate was made for the sole purpose of fast encryption on a single plaintext, as is the case for NetNTLMv1. As such, only encryption on a single plaintext is supported at the moment (e.g only the key part is bitsliced).

## Benchmarks (Ryzen 9 7945HX Laptop CPU)

| Type          | Hashrate   |
| ------------- | ---------- |
| Normal DES    | ~3.88MH/s  |
| Bitsliced DES | ~44.64MH/s |

## Further improvements

- [x] Use CPU SIMD (can give up to 8x performance improvement depending on the register width)
- [ ] Support different plaintexts in bitsliced version
- [ ] Support decryption
