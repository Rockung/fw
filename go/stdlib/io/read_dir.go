package main

import (
  "io/ioutil"
  "fmt"
  "log"
)

func main() {
  files, err := ioutil.ReadDir(".")
  if err != nil {
    log.Fatal(err)
  }

  for _, file := range files {
    fmt.Println(file.Name())
  }
}
