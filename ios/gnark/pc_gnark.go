// Copyright 2015 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// Package hello is a trivial package for gomobile bind example.
package pc_gnark

import (
	"fmt"
	bls12377 "github.com/consensys/gnark-crypto/ecc/bls12-377"
	"math"
	"math/big"
	"pc_gnark/zprize"
	"reflect"
	"runtime"
	"time"

	"github.com/consensys/gnark-crypto/ecc/bls12-377/fr"
	"github.com/consensys/gnark-crypto/ecc/bls12-377/fr/kzg"
)

func RunZprize() {
	zprize.RunZprize()
}

func RunStock() {
	const max_exp = 16
	const max_size = 1 << max_exp
	const divisions = 8

	fmt.Printf("WITHGC / Numcpu: %d\n", runtime.NumCPU())

	// Generate SRS
	fmt.Printf("Allocating SRS_12-377 (%f GB).\n", float64(reflect.TypeOf((*bls12377.G1Affine)(nil)).Elem().Size())*float64(max_size)/1.0e9)
	// srs := kzg.SRS{
	// 	G1: make([]bn254.G1Affine, max_size),
	// };
	// fmt.Printf("Generating random points.\n")
	// start := time.Now()

	// _, _, gen1Aff, gen2Aff := bn254.Generators()
	// srs.G1[0] = gen1Aff
	// srs.G2[0] = gen2Aff
	// srs.G2[1].ScalarMultiplication(&gen2Aff, 2)
	// for i := 1; i < max_size; i++ {
	// 	srs.G1[i].ScalarMultiplication(&gen2Aff, bAlpha)
	// }

	start := time.Now()
	srs, err := kzg.NewSRS(max_size, new(big.Int).SetInt64(1337))
	if err != nil {
		panic(err)
	}
	fmt.Printf("Generating SRS done in %s.\n", time.Now().Sub(start))

	// Create a polynomial
	fmt.Println("Generating scalars.")
	start = time.Now()
	f := make([]fr.Element, max_size)
	for i := 0; i < max_size; i++ {
		f[i].SetRandom()
	}
	fmt.Printf("Generating scalars done in %s.\n", time.Now().Sub(start))

	fmt.Println("size,duration,throughput")

	// Commit polynomial
	for i := 10; i <= max_exp; i++ {
		var size = 1 << i

		var duration = 0.0
		var count = 0

		for duration < 5.0 {
			//runtime.GC()
			start := time.Now()
			_, err := kzg.Commit(f[0:size], srs)
			if err != nil {
				panic(err)
			}
			duration += time.Now().Sub(start).Seconds()
			count += 1
		}
		duration /= float64(count)

		throughput := float64(size) / duration
		fmt.Printf("%d,%f,%f\n", size, duration, throughput)

		var base_size = size
		if i < max_exp {
			for j := 1; j < divisions; j++ {
				var size = int(float64(base_size) * math.Pow(2.0, float64(j)/float64(divisions)))

				var duration = 0.0
				var count = 0

				for duration < 5.0 {
					//runtime.GC()
					start := time.Now()
					_, err := kzg.Commit(f[0:size], srs)
					if err != nil {
						panic(err)
					}
					duration += time.Now().Sub(start).Seconds()
					count += 1
				}
				duration /= float64(count)

				throughput := float64(size) / duration
				fmt.Printf("%d,%f,%f\n", size, duration, throughput)

			}
		}
	}
	fmt.Printf("Done!\n")
}
