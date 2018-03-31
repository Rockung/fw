package main

import "fmt"
import m "pkg/math" // package alias

// 1. add the directory of the file to GOPATH
// 2. move the library files into ***src*** accordingly
func main() {
  xs := []float64{1,2,3,4}
  avg := m.Average(xs)
  fmt.Println(avg)
}
