package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func (p Person) age(age int) Person {
	p.Age = age
	return p
}

func (p Person) checkAge() {
	message := ""

	if p.Age != 0 {
		message = fmt.Sprintf("%s is %d years old.", p.Name, p.Age)
	} else {
		message = fmt.Sprintf("Who knows how old %s is.", p.Name)
	}

	fmt.Println(message)
}

func newPerson(name string) Person {
	return Person{Name: name}
}

func main() {
	newPerson("Sally").age(27).checkAge()
	newPerson("Bill").checkAge()
	newPerson("Pedro").age(34).age(23).checkAge()
}
