# Polynomial Commitment Benchmark

## Machines

* `m1-max`: Apple M1 Max on a Macbook Pro 16".
* `hpc6a`: AWS EC2 `hpc6a.48xlarge` instance.




 p-1 of BLS12-384 factors as
2^32 * 3 * 11 * 19 * 10177 * 125527 * 859267 * 906349^2 * 2508409 * 2529403 * 52437899 * 254760293^2
and BN254 as
2^28 * 3^2 * 13 * 29 * 983 * 11003 * 237073 * 405928799 * 1670836401704629 * 13818364434197438864469338081
Goldilocks as
2^32 * 3 * 5 * 17 * 257 * 65537
(edited)
10:26
So they can go up to 2^32 and 2^28 respectively before you run out of (power-of-two) roots of unity.


BLST on hpc6a can go up to 2^30 before running out of memory.

