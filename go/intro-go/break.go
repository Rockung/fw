package main

import (
  "fmt"
)

func main() {
out:
  for {
    for {
      break out
      fmt.Println("Couldn't go here")
    }
  }

  fmt.Println("Done!")
}
