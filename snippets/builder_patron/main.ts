class Person {
  public name: string;
  public age: number | undefined;

  constructor(name: string, age?: number) {
    this.name = name;
    this.age = age;
  }

  static new(name: string) {
    return new Person(name);
  }

  setAge(age: number) {
    this.age = age;
    return this;
  }

  checkAge() {
    console.log(
      this.age
        ? `${this.name} is ${this.age} years old.`
        : `Who knows how old ${this.name} is?`,
    );
  }
}

Person.new("Sally").setAge(27).checkAge();
Person.new("Bill").checkAge();
Person.new("Pedro").setAge(34).setAge(23).checkAge();
