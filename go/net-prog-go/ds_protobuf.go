package main

import (
	"fmt"
	"github.com/golang/protobuf/proto"
	"os"
	"person"
)

// Protocol Buffers Language Guide
// https://developers.google.com/protocol-buffers/docs/proto

// Installing and compiling protocol buffers
// brew install protobuf
// go get -u github.com/golang/protobuf/protoc-gen-go
// protoc --go_out=src/person personv3.proto

func main() {
	name := person.Person_Name{
		Family:   "newmarch",
		Personal: "jan",
	}
	email1 := person.Person_Email{
		Kind:    "home",
		Address: "jan@newmarch.name",
	}
	email2 := person.Person_Email{
		Kind:    "work",
		Address: "j.newmarch@boxhill.edu.au",
	}
	emails := []*person.Person_Email{&email1, &email2}

	p := person.Person{
		Name:  &name,
		Email: emails,
	}

	fmt.Println(p)
	data, err := proto.Marshal(&p)
	checkError(err)

	newP := person.Person{}
	err = proto.Unmarshal(data, &newP)
	checkError(err)

	fmt.Println(newP)
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}
}
