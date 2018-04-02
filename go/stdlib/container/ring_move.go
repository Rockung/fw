package main

import (
  "fmt"
  "container/ring"
)

func main() {
  r := ring.New(5)

  n := r.Len()
  for i := 0; i < n; i++ {
    r.Value = i
    r = r.Next()
  }

  // move the pointer forward by three steps
  r = r.Move(3)
  r.Do(func(p interface{}) {
    fmt.Println(p.(int))
  })
}
