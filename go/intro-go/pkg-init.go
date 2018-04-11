package main

import (
  "fmt"
)

var (
  a = c + b + d
  b = f()
  c = f()
  d = 3
)

func f() int {
  d++
  return d
}

func main() {
  fmt.Println("a =", a)
  fmt.Println("b =", b)
  fmt.Println("c =", c)
  fmt.Println("d =", d)
}
