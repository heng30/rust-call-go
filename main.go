package main

/*
#include <stdlib.h>
*/
import "C"
import "fmt"

//export GoAdd
func GoAdd(a, b int) int {
	return a + b
}

//export SayHi
func SayHi() {
	fmt.Println("Say hi from Go")
}

func main() {
	fmt.Println("libgo.so")
}
