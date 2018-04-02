package main

import (
  "fmt"
  "encoding/hex"
)

func main() {
  src := []byte("Hello Gopher!")
  dst := make([]byte, hex.EncodedLen(len(src)))
  hex.Encode(dst, src)
  fmt.Printf("%s\n", dst)

  encodedStr := hex.EncodeToString(src)
  fmt.Printf("%s\n", encodedStr)
}
