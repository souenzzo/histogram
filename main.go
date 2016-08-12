package main

import (
	"fmt"

	"github.com/souenzzo/histogram/hist"
)

func main() {
	data := []float64{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 50, -10}
	step := float64(3.2)

	H := hist.New(step)
	for _, v := range data {
		H.Add(v)
	}
	fmt.Println(H.Kmap())
	fmt.Println(H.Imap())
	fmt.Println(H.Krecords(
		func(i int64, k float64, v uint64) []string {
			return []string{
				fmt.Sprintf("%.2f", k),
				fmt.Sprintf("%d", v),
			}
		}))
}
