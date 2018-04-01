package main

// concurrency: making progress on more than one task simultaneously
// Go has rich support for concurrency using goroutines and channels
// To create a goroutine, use the keyword go followed by a function invocation
// Goroutines are lightweight and we can easily create thousands of them
import (
  "fmt"
  "time"
  "math/rand"
)

func f(n int) {
  for i := 0; i < 10; i++ {
    fmt.Println(n, ":", i)
    amt := time.Duration(rand.Intn(250))
    time.Sleep(time.Millisecond * amt)
  }
}

func main() {
  for i := 0; i < 10; i++ {
    go f(0)
  }

  var input string
  fmt.Scanln(&input)
}
