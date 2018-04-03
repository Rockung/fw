package main

import (
  "fmt"
  "reflect"
)

type S struct {
  F string `species:"gopher" color:"blue"`
}

func main() {
  s := S{}
  st := reflect.TypeOf(s)
  field := st.Field(0)
  fmt.Println(field.Tag.Get("color"), field.Tag.Get("species"))
}
