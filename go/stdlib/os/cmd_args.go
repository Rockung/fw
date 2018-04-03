package main

import (
  "os"
  "fmt"
)

func main() {
  // arguments of the command line
  for _, arg := range os.Args {
    fmt.Println(arg)
  }
  // the environment
  for _, env := range os.Environ() {
    fmt.Println(env)
  }
  // the absolute path
  path, _ := os.Executable()
  fmt.Println(path)
  //
  fmt.Println(os.ExpandEnv("$USER lives in ${HOME}"))
  //
  fmt.Printf("%s lives in %s.\n", os.Getenv("USER"), os.Getenv("HOME"))
}
