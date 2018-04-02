package main

// Use a Scanner with a custom split function to parse a comma-separated
// list with an empty final value

import (
  "os"
  "fmt"
  "bufio"
  "strings"
)

func main() {
  const input = "1,2,3,4"
  scanner := bufio.NewScanner(strings.NewReader(input))
  onComma := func(data []byte, atEOF bool) (advance int, token []byte, err error) {
    for i := 0; i < len(data); i++ {
      if data[i] == ',' {
        return i+1, data[:i], nil
      }
    }
    return 0, data, bufio.ErrFinalToken
  }
  scanner.Split(onComma)
  for scanner.Scan() {
    fmt.Printf("%q ", scanner.Text())
  }
  if err := scanner.Err(); err != nil {
    fmt.Fprintln(os.Stderr, "reading input:", err)
  }
}
