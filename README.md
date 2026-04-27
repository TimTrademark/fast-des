# fast-des 🚀

This crate implements DES using a bitsliced implementation. I set out to teach myself both the inner workings of DES and [bitslicing](https://timtaubert.de/blog/2018/08/bitslicing-an-introduction/). The bitsliced implementation achieved more than a 100x in performance for 512 bit (AVX) registers.

## Features

This crate was made for the sole purpose of fast encryption on a single plaintext, as is the case for NetNTLMv1. As such, only encryption on a single plaintext is supported at the moment (e.g only the key part is bitsliced).

## Benchmarks (Ryzen 9 7945HX Laptop CPU)

| Type                                          | Hashrate    |
| --------------------------------------------- | ----------- |
| Normal DES                                    | ~3.88MH/s   |
| Bitsliced DES (AVX)                           | ~747.21MH/s |
| Bitsliced DES Parallel (AVX 32 threads)       | ~4.02GH/s   |
| Bitsliced NetNTLMv1 (AVX)                     | ~675.48MH/s |
| Bitsliced NetNTLMv1 Parallel (AVX 32 threads) | ~3.86GH/s   |

## Further improvements

- [x] Use CPU SIMD (can give up to 8x performance improvement depending on the register width)
- [ ] Support different plaintexts in bitsliced version
- [ ] Support decryption

## Usage

DES encryption:

```rust
let k = 0x133457799BBCDFF1u64;
//fill up array with 512 keys (groups of 64). for simplcity we fill the full array with the same key in this example.
let mut keys = [[k; 64]; 8];
//encrypt with 512 keys at once
let ciphertexts = bitsliced_des_simd(0x0123456789ABCDEF, &mut keys);
//all outputs are encrypted ciphertexts
assert_eq!(ciphertexts[0][0], 0x85E813540F0AB405);
assert_eq!(ciphertexts[0][63], 0x85E813540F0AB405);
```

NetNTLMv1 hash:

```rust
//netntlmv1 works on 56bit keys
let k = 0x8846F7EAEE8FB1u64;
let keys = [[k; 64]; 8];
//netntlmv1 hash with 512 keys at once
let ciphertexts = bitsliced_netntlmv1_simd(0x1122334455667788, &keys);
assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
assert_eq!(ciphertexts[0][63], 0x727B4E35F947129E);
```

NetNTLMv1 hash AVX (this is the fastest but requires AVX support from your CPU):

```rust
//AVX usage requires unsafe blocks
//beware, this may crash if your CPU does not support AVX
unsafe {
    let k = 0x8846F7EAEE8FB1u64;
    let keys = [[k; 64]; 8];
    //netntlmv1 hash with 512 keys at once using AVX
    let ciphertexts = bitsliced_netntlmv1_simd_avx_512(&keys);
    assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
    assert_eq!(ciphertexts[0][63], 0x727B4E35F947129E);
}
```
