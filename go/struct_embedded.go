package main

import "fmt"

func main() {
  p := Person{"Rock"}
  p.Talk()
  android := Android{
    Person{"Android"},  // A comma is needed
    "Model 5",          // A comma is needed
  }
  android.Talk()
}

type Person struct {
  Name string
}

func (p *Person) Talk() {
  fmt.Println("Hi, my name is", p.Name)
}

type Android struct {
  Person            // no need a comma
  Model string      // no need a commo
}
