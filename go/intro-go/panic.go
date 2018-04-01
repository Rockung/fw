package main

import "fmt"

func main() {
  panic("PANIC")
  str := recover() // this will never happen
  fmt.Println(str)
}

