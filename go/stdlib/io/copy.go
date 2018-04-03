package main

import (
  "io"
  "os"
  "log"
  "strings"
)

func main() {
  r := strings.NewReader("some io.Reader stream to be read\n")
  if _, err := io.Copy(os.Stdout, r); err != nil {
    log.Fatal(err)
  }
}
