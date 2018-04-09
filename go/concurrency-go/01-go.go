package main

import (
	"fmt"
	"time"
)

type Job struct {
	i    int
	max  int
	text string
}

func outputText(j *Job) {
	for j.i < j.max {
		time.Sleep(1 * time.Millisecond)
		fmt.Println(j.text)
		j.i++
	}
}

func main() {
	hello := Job{0, 3, "hello"}
	world := Job{0, 5, "world"}

	go outputText(&hello)
	outputText(&world)

	// go outputText(&world)
	// for {} // to prevent the main thread exits
}
