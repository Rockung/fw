package main

// This example creates a PriorityQueue with some items, adds and manipulates
// an item, and then removes the items in priority order.

import (
  "fmt"
  "container/heap"
)

func main() {
  items := map[string]int{
    "banana": 3,
    "appple": 2,
    "pear":   4,
  }

  // 1. make a slice of PriorityQueue from a map
  // 2. initialize the slice as a heap
  pq := make(PriorityQueue, len(items))
  i := 0
  for value, priority := range items {
    pq[i] = &Item{
      value:    value,
      priority: priority,
      index:    i,
    }
    i++
  }
  heap.Init(&pq)

  // 1. push an item into the heap
  // 2. update the item value/priority
  item := &Item{
    value:    "orange",
    priority: 1,
  }
  heap.Push(&pq, item)
  pq.update(item, item.value, 5)

  // pop the items in the heap to see if they are in order
  // **x.(T)**: type assertion which asserts that x is not nil and that the
  // value stored in x is of type T.
  for pq.Len() > 0 {
    item := heap.Pop(&pq).(*Item)
    fmt.Printf("%.2d:%s\n", item.priority, item.value)
  }
}

type Item struct {
  value string
  priority int
  index int
}

// a slice of pointers to Item
type PriorityQueue []*Item

// sort.Interface
func (pq PriorityQueue) Len() int { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool {
  // pop the highest, not the lowest that in descendant order
  return pq[i].priority > pq[j].priority
}
func (pq PriorityQueue) Swap(i, j int) {
  pq[i], pq[j] = pq[j], pq[i]
  pq[i].index = i
  pq[j].index = j
}

// heap.Interface
func (pq *PriorityQueue) Push(x interface{}) {
  n := len(*pq)
  item := x.(*Item)
  item.index = n
  *pq = append(*pq, item)
}
func (pq *PriorityQueue) Pop() interface{} {
  old := *pq
  n := len(old)
  item := old[n-1]
  item.index = -1
  *pq = old[0 : n-1]
  return item
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) update(item *Item, value string, priority int) {
  item.value = value
  item.priority = priority
  heap.Fix(pq, item.index)
}
