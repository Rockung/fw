package main

import (
	"fmt"
	"os"
	"text/template"
)

type Person struct {
	Name   string
	Age    int
	Emails []string
	Jobs   []*Job
}

type Job struct {
	Employer string
	Role     string
}

/*
const templ = `The name is {{.Name}}.
The age is {{.Age}}.
{{range .Emails}}
  An email is {{.}}
{{end}}
{{range .Jobs}}
  An employer is {{.Employer}}
  and the role is {{.Role}}
{{end}}
`
*/

const templ = `The name is {{.Name}}.
The age is {{.Age}}.
{{range .Emails}}
  An email is {{.}}
{{end}}
{{with .Jobs}}
  {{range .}}
    An employer is {{.Employer}}
    and the role is {{.Role}}
  {{end}}
{{end}}
`

func main() {
	job1 := Job{Employer: "Box Hill Institue", Role: "Director, Commerce and ICT"}
	job2 := Job{Employer: "Canberra University", Role: "Adjunct Professor"}

	person := Person{
		Name:   "jan",
		Age:    66,
		Emails: []string{"jan@newmarch.name", "jan.newmarch@gmail.com"},
		Jobs:   []*Job{&job1, &job2},
	}

	t := template.New("Person-template")
	t, err := t.Parse(templ)
	checkError(err)

	err = t.Execute(os.Stdout, person)
	checkError(err)
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error ", err.Error())
		os.Exit(1)
	}
}
