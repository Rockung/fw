package main

// Use a Scanner to implement a simple word-count utility by scanning
// the input as a sequence of space-delimited tokens.

import (
  "os"
  "fmt"
  "bufio"
  "strings"
)

func main() {
  const input = "Now is the winter of our discontent,\nMade glorious summer by this sun of York.\n"
  scanner := bufio.NewScanner(strings.NewReader(input))
  scanner.Split(bufio.ScanWords)
  count := 0
  for scanner.Scan() {
    count++
    fmt.Println(scanner.Text())
  }
  if err := scanner.Err(); err != nil {
    fmt.Fprintln(os.Stderr, "reading input:", err)
  }
  fmt.Printf("%d\n", count)
}
