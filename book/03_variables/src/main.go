package main

import "fmt"

const THREE_HOURS_IN_SECONDS = 60 * 60 * 3

func main() {
	// mutable by default
	x := 5

	fmt.Printf("The value of x is: %d\n", x)

	x = 6

	{
		x := x * 2
		fmt.Printf("The value of x in the inner scope is: %d\n", x)
	}

	fmt.Printf("The pointer value of x is: %v\n", &x)

	/* fmt.Println(&x)
	fmt.Println(&x)
	fmt.Println(*&x)
	*/

	fmt.Println(THREE_HOURS_IN_SECONDS)
}
