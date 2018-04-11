package main

import (
  "fmt"
)

const (
  c0 = iota
  c1
  c2
  c3 = iota
  c4
)

func main() {
  fmt.Println("c0:", c0)
  fmt.Println("c1:", c1)
  fmt.Println("c2:", c2)
  fmt.Println("c3:", c3)
  fmt.Println("c4:", c4)
}
