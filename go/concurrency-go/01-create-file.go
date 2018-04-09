package main

import (
  "fmt"
  "time"
  "io/ioutil"
)

type Job struct {
  i int
  max int
  text string
}

func outputText(j *Job) {
  fileName := j.text + ".txt"
  fileContents := ""

  for j.i < j.max {
    time.Sleep(1 * time.Millisecond)
    fileContents += j.text
    fmt.Println(j.text)
    j.i++
  }

  err := ioutil.WriteFile(fileName, []byte(fileContents), 0644)
  if (err != nil) {
    panic("Something went awry")
  }
}

func main() {
  hello := Job{0, 3, "hello"}
  world := Job{0, 5, "world"}

  go outputText(&hello)
  go outputText(&world)

  for {} // to prevent the main thread exits
}
