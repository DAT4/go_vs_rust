package main

import (
	"fmt"
	"runtime"
	"time"
)

const PI25DT float64 = 3.141592653589793238462643
const INTERVALS int = 10000000

func main() {
	cpus := runtime.NumCPU()
	for i := 1; i <= cpus; i++ {
		for j := 0; j < 20; j++ {
			fmt.Println(run(i))
		}
	}
}

func run(cpus int) string {
	var (
		time1     = time.Now()
		ch        = make(chan float64)
		intervals = INTERVALS / cpus
		dx        = 1.0 / float64(INTERVALS)
	)

	for i := 1; i <= cpus; i++ {
		go func(cpu int) {
			var (
				innerSum float64
				x        float64
				end      = intervals * cpu
				start    = end - intervals
			)
			for j := start; j <= end; j++ {
				x = dx * (float64(j) - 0.5)
				innerSum += 4.0 / (1.0 + x*x)
			}
			ch <- innerSum
		}(i)
	}

	var sum float64

	for i := cpus; i > 0; i-- {
		sum += <-ch
	}

	var (
		pi    = dx * sum
		time2 = time.Since(time1).Seconds()
	)

	return fmt.Sprintf("%d, %.24f, %.24f, %.24f", cpus, pi, PI25DT-pi, time2)
}
