package main

import (
  "io"
  "io/ioutil"
  "log"
  "fmt"
  "bytes"
  "strings"
)

func main() {
  r := strings.NewReader("some io.Reader stream to be read\n")
  var buf bytes.Buffer
  tee := io.TeeReader(r, &buf)

  printall := func(r io.Reader) {
    b, err := ioutil.ReadAll(r)
    if err != nil {
      log.Fatal(err)
    }

    fmt.Printf("%s", b)
  }

  printall(tee)
  printall(&buf)
}
