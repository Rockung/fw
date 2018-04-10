package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"sync"
)

var writer chan bool
var rwLock sync.RWMutex

func main() {
	writer = make(chan bool)
	for i := 0; i < 10; i++ {
		go writeFile(i)
	}
	fmt.Println("Write some: ", <-writer)
	fmt.Println("Done!")
}

func writeFile(i int) {
	rwLock.RLock()
	fmt.Println("Write: ", i)
	ioutil.WriteFile("test.txt",
		[]byte(strconv.FormatInt(int64(i), 10)), 0x777)
	rwLock.RUnlock()

	writer <- true
}
