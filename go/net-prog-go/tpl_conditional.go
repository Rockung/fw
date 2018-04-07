package main

import (
	"fmt"
	"os"
	"text/template"
)

type Person struct {
	Name   string
	Emails []string
}

const templ = `{"Name": "{{.Name}}",
	"Emails":[
{{range $index, $elemt := .Emails}}
	{{if $index}},"{{$elemt}}"{{else}}"{{$elemt}}"{{end}}
{{end}}
	]
}
`

func main() {
	person := Person{
		Name:   "jan",
		Emails: []string{"jan@newmarch.name", "jan.newmarch@gmail.com"},
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
