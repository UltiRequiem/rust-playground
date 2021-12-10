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
	fmt.Printf("The double of %d is %d.", 10, double(10))
}
