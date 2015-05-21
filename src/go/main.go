package main

import "C"

//export Multiply
func Multiply(a, b int) int {
	return a * b
}

func main() {
}
