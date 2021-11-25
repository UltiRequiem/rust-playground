package main

import "fmt"

type Rectangle struct {
	height int
	width  int
}

func newRectangle(height, width int) *Rectangle {
	return &Rectangle{height, width}
}

func (r *Rectangle) area() int {
	return r.height * r.width
}

func main() {
	rect := newRectangle(50, 60)
	fmt.Printf("The area of the rectangle is %d square pixels.", rect.area())
}
