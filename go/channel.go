package main

// Channels provide a way for two goroutines to communicate with each
// other and synchronize their execution
// A channel type is represented with the keyword chan followed by the
// type of the things are passed on the channel
// The left arrow operator(<-) is used to send and receive message on
// the Channel
// Channel direction: we can specify a direction on a channel type
//    chan<-: the channel is only allowed to send to it
//    <-chan: the channl is only allowed to receive from it
//      chan: the channel is bidirectional

import (
  "fmt"
  "time"
)

func main() {
  c := make(chan string)

  go pinger(c)
  go ponger(c)
  go printer(c)

  var input string
  fmt.Scanln(&input)
}

func pinger(c chan<- string) {
  for i := 0; ; i++ {
    c <- "ping"
  }
}

func ponger(c chan<- string) {
  for i := 0; ; i++ {
    c <- "pong"
  }
}

func printer(c <-chan string) {
  for {
    msg := <- c
    fmt.Println(msg)
    time.Sleep(time.Second * 1)
  }
}
