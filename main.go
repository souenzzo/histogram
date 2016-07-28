package main

import (
	"fmt"
	_ "math"
)

type histogram struct {
	step float64
	Map  map[int64]uint64
}

func New(step float64) *histogram {
	return &histogram{
		step: step,
		Map:  make(map[int64]uint64),
	}
}

func Key(step, value float64) (k int64) {
	k = 0
	for {
		if (float64(k) * step) > value {
			return k - 1
		}
		k = k + int64(1)
	}
}

func (H *histogram) Append(value float64) {
	k := Key(H.step, value)
	H.Map[k]++
}

func main() {
	data := []float64{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 50}
	step := float64(3.2)

	H := New(step)
	for _, v := range data {
		H.Append(v)
	}
	fmt.Println(H.Map)
	for k, v := range H.Map {
		fmt.Printf("%.2f - %d\n", float64(k)*step, v)
	}
}
