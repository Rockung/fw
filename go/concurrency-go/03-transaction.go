package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"time"
)

var balance int
var transNo int
var mutex sync.Mutex

func main() {
	rand.Seed(time.Now().Unix())
	runtime.GOMAXPROCS(2)
	var wg sync.WaitGroup
	tranChan := make(chan bool)

	balance = 1000
	transNo = 0
	fmt.Println("Starting balance: $", balance)

	wg.Add(1)
	for i := 0; i < 100; i++ {
		go func(i int, tranChan chan (bool)) {
			amount := rand.Intn(25)
			transaction(amount)
			if i == 99 {
				tranChan <- true
			}
		}(i, tranChan)
	}

	go transaction(0)
	select {
	case <-tranChan:
		fmt.Println("Transactions finished")
		wg.Done()
	}

	wg.Wait()
	close(tranChan)
	fmt.Println("Final balance: $", balance)
}

func transaction(amt int) bool {
	mutex.Lock()
	approved := false
	if (balance - amt) < 0 {
		approved = false
	} else {
		approved = true
		balance = balance - amt
	}

	approvedText := "declined"
	if approved == true {
		approvedText = "approved"
	}

	transNo = transNo + 1
	fmt.Println(transNo, "Transaction for $", amt, approvedText)
	fmt.Println("\tRemaining balance $", balance)
	mutex.Unlock()

	return approved
}
