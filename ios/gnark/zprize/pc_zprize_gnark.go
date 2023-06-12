package zprize

import (
	"fmt"
	"github.com/consensys/gnark-crypto/ecc"
	bls12377 "github.com/gbotrel/zprize-mobile-harness/msm/bls12-377"
	"github.com/gbotrel/zprize-mobile-harness/msm/bls12-377/fr"
	"math"
	"reflect"
	"runtime"
	"time"

	"math/big"
)

type PK struct {
	powers []bls12377.G1EdMSM
}

func NewPK(size int, base *big.Int) (PK, error) {
	var g1Gen bls12377.G1Jac
	var g1GenAff bls12377.G1Affine
	g1Gen.X.SetString("81937999373150964239938255573465948239988671502647976594219695644855304257327692006745978603320413799295628339695")
	g1Gen.Y.SetString("241266749859715473739788878240585681733927191168601896383759122102112907357779751001206799952863815012735208165030")
	g1Gen.Z.SetString("1")

	g1GenAff.FromJacobian(&g1Gen)
	var pk PK
	pk.powers = make([]bls12377.G1EdMSM, size)
	alpha := new(fr.Element).SetBigInt(base)
	alphas := make([]fr.Element, size-1)
	alphas[0] = *alpha
	for i := 1; i < len(alphas); i++ {
		alphas[i].Mul(&alphas[i-1], alpha)
	}
	g1s := bls12377.BatchScalarMultiplicationG1(&g1GenAff, alphas)
	g1sEdMSM := bls12377.BatchFromAffineSWC(g1s)
	copy(pk.powers[1:], g1sEdMSM)
	return pk, nil
}

func Commit(p []fr.Element, pk *PK) (bls12377.G1Affine, error) {
	var result bls12377.G1EdExtended
	result.MultiExp(pk.powers[:len(p)], p, ecc.MultiExpConfig{})
	return *new(bls12377.G1Affine).FromExtendedEd(&result), nil
}

func RunZprize() {
	fmt.Printf("Running Zprize\n")
	const max_exp = 16
	const max_size = 1 << max_exp
	const divisions = 8

	fmt.Printf("WITHGC / Numcpu: %d\n", runtime.NumCPU())

	// Generate SRS
	fmt.Printf("Allocating SRS_12-377 (%f GB).\n", float64(reflect.TypeOf((*bls12377.G1Affine)(nil)).Elem().Size())*float64(max_size)/1.0e9)

	start := time.Now()
	pk, err := NewPK(max_size, new(big.Int).SetInt64(1337))
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

	fmt.Println("size,duration,throughput,runs")

	// Commit polynomial
	for i := 10; i <= max_exp; i++ {
		var size = 1 << i

		var duration = 0.0
		var count = 0

		for duration < 5.0 || count < 10 {
			//runtime.GC()
			start := time.Now()
			_, err := Commit(f[0:size], &pk)
			if err != nil {
				panic(err)
			}
			duration += time.Now().Sub(start).Seconds()
			count += 1
		}
		duration /= float64(count)

		throughput := float64(size) / duration
		fmt.Printf("%d,%f,%f,%d\n", size, duration, throughput, count)

		var base_size = size
		if i < max_exp {
			for j := 1; j < divisions; j++ {
				var size = int(float64(base_size) * math.Pow(2.0, float64(j)/float64(divisions)))

				var duration = 0.0
				var count = 0

				for duration < 5.0 || count < 10 {
					//runtime.GC()
					start := time.Now()
					_, err := Commit(f[0:size], &pk)
					if err != nil {
						panic(err)
					}
					duration += time.Now().Sub(start).Seconds()
					count += 1
				}
				duration /= float64(count)

				throughput := float64(size) / duration
				fmt.Printf("%d,%f,%f,%d\n", size, duration, throughput, count)

			}
		}
	}
	fmt.Printf("Done!\n")
}
