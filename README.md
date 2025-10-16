# fast-des 🚀

This crate implements DES using a bitsliced implementation. I set out to teach myself both the inner workings of DES and [bitslicing](https://timtaubert.de/blog/2018/08/bitslicing-an-introduction/). The bitsliced implementation achieved a 10x in performance.

## Features

This crate was made for the sole purpose of fast encryption on a single plaintext, as is the case for NetNTLMv1. As such, only encryption on a single plaintext is supported at the moment (e.g only the key part is bitsliced).

## Benchmarks (Intel i5-8500)

| Type | Hashrate |
| Normal DES | 457.29KH/s |
| Bitsliced DES | 5.56MH/s |

## Further improvements

- [] Use CPU SIMD (can give up to 8x performance improvement depending on the register width)
- [] Optimize ILP in encrypt_optimized
- [] Support different plaintexts in bitsliced version
- [] Support decryption
