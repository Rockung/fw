package main

import (
  "fmt"
  "context"
)

// func WithCancel(parent Context) (ctx Context, cancel CancelFunc)

// WithCancel returns a copy of parent with a new Done channel. The returned
// context's Done channel is closed when the returned cancel function is called
// or when the parent context's Done channel is closed, whichever happens first.

// Canceling this context releases resources associated with it, so code should
// call cancel as soon as the operations running in this Context complete.

func main() {
  // 1. creates and returns a channel
  // 2. creates goroutine to send integers into the channel in background
  // 3. the caller uses ctx.Done() to cancel the goroutine
  gen := func(ctx context.Context) <-chan int {
    dst := make(chan int)
    n := 1
    go func() {
      for {
        select {
        case <-ctx.Done():
          return
        case dst <- n:
          n++
        }
      }
    }()
    return dst
  }

  // Background returns a non-nil, empty Context. It is never canceled, has no
  // values, and has no deadline. It is typically used by the main function,
  // initialization, and tests, and as the top-level Context for incoming
  // requests.
  ctx, cancel := context.WithCancel(context.Background())
  defer cancel() // notify the context to cancel the background goroutine

  // For channels, the iteration values produced are the successive values sent
  // on the channel until the channel is closed. If the channel is nil, the
  // range expression blocks forever
  // @see #Built-in functions in language specification
  for n := range gen(ctx) {
    fmt.Println(n)
    if n == 5 {
      break
    }
  }
}
