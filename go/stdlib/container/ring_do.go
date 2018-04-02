package main

// func (r *Ring) Do(f func(interface{}))
// Do calls function f on each element of the ring, in forward order.
// The behavior of Do is undefined if f changes *r.

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

  r.Do(func(p interface{}){
    fmt.Println(p.(int))
  })

  r.Do(func(p interface{}){
    fmt.Println(p.(int))
  })
}
