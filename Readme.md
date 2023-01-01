# Polynomial Commitment Benchmark


## Machines

* `m1-max`: Apple M1 Max on a Macbook Pro 16" with 64GB memory.
* `hpc6a`: AWS EC2 `hpc6a.48xlarge` instance.
* `a14`: Apple iPhone 13 Pro with 6GB DDR and 512 GB NVMe.


## Algorithms

I'm aiming for settings that maximize prover performance. I am also not interested in zero-knowledge, as that can be achieved more efficiently in later layers of recursion.

* `blst`: Compute an MSM of scalars and BLS12-381 𝔾₁ points.
* `plonky2-keccak`: Compute a Keccak Merkle tree of a rate 2 low degree expansion over Goldilocks.
* `plonky2-poseidon`: Same, but using Goldilocks-Poseidon hash function.
* `halo2`: Compute and MSM of scalars and Pallas points.

**To do.**

* `pse`: https://github.com/privacy-scaling-explorations/halo2

## Limits

BLST on m1-max can go up to 2^28 before running out of memory.
BLST on hpc6a can go up to 2^30 before running out of memory.
This is expected since the input data will fill the memory.

Plonky2 on m1-max can go to 2^28 before running out of memory.
Plonky2 on hpc6a can go to 2^30 before running out of memory.
This is surprising because the input data is far short of filling the memory.


 p-1 of BLS12-384 factors as
2^32 * 3 * 11 * 19 * 10177 * 125527 * 859267 * 906349^2 * 2508409 * 2529403 * 52437899 * 254760293^2
and BN254 as
2^28 * 3^2 * 13 * 29 * 983 * 11003 * 237073 * 405928799 * 1670836401704629 * 13818364434197438864469338081
Goldilocks as
2^32 * 3 * 5 * 17 * 257 * 65537
(edited)
10:26
So they can go up to 2^32 and 2^28 respectively before you run out of (power-of-two) roots of unity.
