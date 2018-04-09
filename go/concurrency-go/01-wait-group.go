package main

import (
  "fmt"
  "time"
  "sync"
)

type Job struct {
  i int
  max int
  text string
}

func outputText(j *Job, goGroup *sync.WaitGroup) {
  for j.i < j.max {
    time.Sleep(1 * time.Millisecond)
    fmt.Println(j.text)
    j.i++
  }
  goGroup.Done() // signal Done message to the waiting thread
}

func main() {
  hello := Job{0, 3, "hello"}
  world := Job{0, 5, "world"}
  goGroup := new(sync.WaitGroup)

  fmt.Println("Starting ...")
  go outputText(&hello, goGroup)
  go outputText(&world, goGroup)

  goGroup.Add(2) // set the number of goroutines that need to wait
  goGroup.Wait()
}
