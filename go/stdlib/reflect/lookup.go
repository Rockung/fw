package main

import (
  "fmt"
  "reflect"
)

type S struct {
  F0 string `alias:"field_0"`
  F1 string `alias:""`
  F2 string
}

func main() {
  s := S{}
  st := reflect.TypeOf(s)
  for i := 0; i < st.NumField(); i++ {
    field := st.Field(i)
    if alias, ok := field.Tag.Lookup("alias"); ok {
      if alias == "" {
        fmt.Println("(blank)")
      } else {
        fmt.Println(alias)
      }
    } else {
      fmt.Println("(not specified)")
    }
  }
}
