package main

import (
	"fmt"
  "sync"
)

var comm = make(chan bool)
var done = make(chan bool)
var wg = new(sync.WaitGroup)

func producer() {
	for i := 0; i < 10; i++ {
		comm <- true
	}

	done <- true
}

func consumer() {
	for {
		select {
		case msg := <-comm:
			fmt.Println("Received: ", msg)
		case <-done:
      wg.Done()
			break
		}
	}
}

func main() {
  wg.Add(1)

	go producer()
	go consumer()

  wg.Wait()
  fmt.Println("All done!")
}
