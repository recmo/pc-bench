// Copyright 2015 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// Package hello is a trivial package for gomobile bind example.
package pc_gnark

import (
	"fmt"
	"math"
	"math/big"
	"reflect"
	"time"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr/kzg"
)

func Run() {
	const maxExp = 22
	const maxSize = 1 << maxExp
	const divisions = 8

	// Generate SRS
	fmt.Printf("Allocating SRS (%f GB).\n", float64(reflect.TypeOf((*bn254.G1Affine)(nil)).Elem().Size())*float64(maxSize)/1.0e9)

	start := time.Now()
	srs, err := kzg.NewSRS(maxSize, new(big.Int).SetInt64(1337))
	if err != nil {
		panic(err)
	}
	fmt.Printf("Generating SRS done in %s.\n", time.Now().Sub(start))

	// Create a polynomial
	fmt.Println("Generating scalars.")
	start = time.Now()
	f := make([]fr.Element, maxSize)
	for i := 0; i < maxSize; i++ {
		_, err := f[i].SetRandom()
		if err != nil {
			panic(err)
		}
	}
	fmt.Printf("Generating scalars done in %s.\n", time.Now().Sub(start))

	fmt.Println("size,duration,throughput")

	// Commit polynomial
	for i := 10; i <= maxExp; i++ {
		var size = 1 << i

		var duration = 0.0
		var count = 0

		for duration < 5.0 {
			start := time.Now()
			_, err := kzg.Commit(f[0:size], srs.Pk)
			if err != nil {
				panic(err)
			}
			duration += time.Now().Sub(start).Seconds()
			count += 1
		}
		duration /= float64(count)

		throughput := float64(size) / duration
		fmt.Printf("%d,%f,%f\n", size, duration, throughput)

		var baseSize = size
		if i < maxExp {
			for j := 1; j < divisions; j++ {
				var size = int(float64(baseSize) * math.Pow(2.0, float64(j)/float64(divisions)))

				var duration = 0.0
				var count = 0

				for duration < 5.0 {
					start := time.Now()
					_, err := kzg.Commit(f[0:size], srs.Pk)
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
