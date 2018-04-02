package main

import (
  "fmt"
  "log"
  "encoding/hex"
)

func main() {
  src := []byte("48656c6c6f20476f7068657221")
  dst := make([]byte, hex.DecodedLen(len(src)))
  n, err := hex.Decode(dst, src)
  if err != nil {
    log.Fatal(err)
  }
  fmt.Printf("%s\n", dst[:n])

  const s = "48656c6c6f20476f7068657221"
  decoded, err := hex.DecodeString(s)
  if err != nil {
    log.Fatal(err)
  }
  fmt.Printf("%s\n", decoded)
}
