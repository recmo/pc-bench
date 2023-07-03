package main

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

func parallelize[I any, O any](op func(I) (O, error), inputs []I) []O {
	outChans := make([]chan O, len(inputs))
	for i, input := range inputs {
		outChans[i] = make(chan O)
		go func(input I, outChan chan O) {
			res, err := op(input)
			if err != nil {
				panic(err)
			}
			outChan <- res
		}(input, outChans[i])
	}
	outputs := make([]O, len(inputs))
	for i, outChan := range outChans {
		outputs[i] = <-outChan
		close(outChan)
	}
	return outputs
}

func main() {
	const maxExp = 28
	const maxSize = 1 << maxExp
	const batchSize = 16
	const divisions = 8

	// Generate SRS
	fmt.Printf("Allocating SRS (%f GB).\n", float64(reflect.TypeOf((*bn254.G1Affine)(nil)).Elem().Size())*float64(maxSize)/1.0e9)
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
	srs, err := kzg.NewSRS(maxSize, new(big.Int).SetInt64(1337))
	if err != nil {
		panic(err)
	}
	fmt.Printf("Generating SRS done in %s.\n", time.Now().Sub(start))

	// Create a polynomial
	fmt.Println("Generating scalars.")
	start = time.Now()
	f := make([][]fr.Element, batchSize)
	for i := 0; i < batchSize; i++ {
		f[i] = make([]fr.Element, maxSize)
	}
	setup := func(inp []fr.Element) (int, error) {
		for i := 0; i < maxSize; i++ {
			inp[i].SetRandom()
		}
		return 0, nil
	}
	parallelize(setup, f)
	println(f[10][20].String())
	fmt.Printf("Generating scalars done in %s.\n", time.Now().Sub(start))

	fmt.Println("size,duration,throughput")

	// Commit polynomial
	for i := 10; i <= maxExp; i++ {
		var size = 1 << i

		var duration = 0.0
		var count = 0

		for duration < 5.0 {
			start := time.Now()
			op := func(inp []fr.Element) (kzg.Digest, error) { return kzg.Commit(inp[0:size], srs) }
			parallelize(op, f)
			duration += time.Now().Sub(start).Seconds()
			count += 1
		}
		duration /= float64(count)

		throughput := float64(size*batchSize) / duration
		fmt.Printf("%d,%f,%f\n", size, duration, throughput)

		var baseSize = size
		if i < maxExp {
			for j := 1; j < divisions; j++ {
				var size = int(float64(baseSize) * math.Pow(2.0, float64(j)/float64(divisions)))

				var duration = 0.0
				var count = 0

				for duration < 5.0 {
					start := time.Now()
					op := func(inp []fr.Element) (kzg.Digest, error) { return kzg.Commit(inp[0:size], srs) }
					parallelize(op, f)
					duration += time.Now().Sub(start).Seconds()
					count += 1
				}
				duration /= float64(count)

				throughput := float64(size) / duration
				fmt.Printf("%d,%f,%f\n", size, duration, throughput)

			}
		}
	}

}
