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

func main() {
    fmt.Println("libgo.so")
}
