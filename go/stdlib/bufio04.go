package main

// Use a Scanner with a custom split function (built by wrapping ScanWords)
// to validate 32-bit decimal input.

import (
  "fmt"
  "bufio"
  "strings"
  "strconv"
)

func main() {
  const input = "1234 12345678901234567890 5678"
  scanner := bufio.NewScanner(strings.NewReader(input))
  split := func(data []byte, atEOF bool) (advance int, token []byte, err error) {
    advance, token, err = bufio.ScanWords(data, atEOF)
    if err == nil && token != nil {
      _, err = strconv.ParseInt(string(token), 10, 32)
    }
    return
  }

  scanner.Split(split)
  for scanner.Scan() {
    fmt.Printf("%s\n", scanner.Text())
  }

  if err := scanner.Err(); err != nil {
    fmt.Printf("Invalid input: %s", err)
  }
}
