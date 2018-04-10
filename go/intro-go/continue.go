package main

import (
  "fmt"
)

func main() {
out:
  for i := 0; i < 10; i++ {
    fmt.Println("Times: ", i)
    for {
      continue out
      fmt.Println("Couldn't go here")
    }
  }

  fmt.Println("Done!")
}
