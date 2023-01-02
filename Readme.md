# Polynomial Commitment Benchmark


## Machines

* `m1-max`: Apple M1 Max on a Macbook Pro 16" with 64GB memory.
* `hpc6a`: AWS EC2 `hpc6a.48xlarge` instance.
* `a14`: Apple iPhone 13 Pro with 6GB DDR and 512 GB NVMe.


## Algorithms

I'm aiming for settings that maximize prover performance. I am also not interested in zero-knowledge, as that can be achieved more efficiently in later layers of recursion.

* `blst`: Compute an MSM of scalars and BLS12-381 𝔾₁ points.
* `plonky2-keccak`: Compute a Keccak Merkle tree of a low degree expansion over Goldilocks. The rate is set to 2 and the FFT root table is pre-computed.
* `plonky2-poseidon`: Same, but using Goldilocks-Poseidon hash function.
* `halo2`: Compute the MSM of scalars and Pallas points.
* `pse`: Compute MSM of BN254 𝔾₁ points using PSE fork of Halo2.
* `gnark` Compute MSM of scalars and BN254 𝔾₁ points using the `kzg.Commit` function.

[]: https://github.com/privacy-scaling-explorations/halo2


**To do.**


## Limits

BLST on m1-max can go up to 2^28 before running out of memory.
BLST on hpc6a can go up to 2^30 before running out of memory.
This is expected since the input data will fill the memory.

Plonky2 on m1-max can go to 2^28 before running out of memory.
Plonky2 on hpc6a can go to 2^30 before running out of memory.
This is surprising because the input data is far short of filling the memory.

Gnark on hpc6a can go to 2^30 before running out of memory.


| Algo | a14 | m1-max | hpc6a |
|------|-----|--------|-------|
| halo2 | 24 | 28 | 31 |


p-1 of BLS12-384 factors as
2^32 * 3 * 11 * 19 * 10177 * 125527 * 859267 * 906349^2 * 2508409 * 2529403 * 52437899 * 254760293^2

and BN254 as
2^28 * 3^2 * 13 * 29 * 983 * 11003 * 237073 * 405928799 * 1670836401704629 * 13818364434197438864469338081


Goldilocks as
2^32 * 3 * 5 * 17 * 257 * 65537


Pasta curves (Halo2)

p - 1:
2^32 × 3 × 463 × 4852402207910482324454106387152561316357015077916052529702775169

q - 1:
2^32 × 3^2 × 1709 × 24859 × 17627503553531704781201602214972145569028026719617221564519

