// Coolest  Version IMO

package main

import "fmt"

func getMultiplier(multiplier int) func(int) int {
	return func(i int) int {
		return i * multiplier
	}
}

func main() {
	double := getMultiplier(2)
	for i := 1; i < 7; i++ {
		fmt.Printf("The double of %d is %d.\n", i, double(i))
	}
}
