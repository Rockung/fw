package main

import (
	"fmt"
	"sync"
)

func main() {
	current := 0
	iterations := 100
	wg := new(sync.WaitGroup)
	mutex := new(sync.Mutex)

	for i := 0; i < iterations; i++ {
		wg.Add(1)
		go func() {
			mutex.Lock()
			current++
			fmt.Println(current)
			mutex.Unlock()
			wg.Done()
		}()
		// wg.Wait()
	}

	wg.Wait()
}
