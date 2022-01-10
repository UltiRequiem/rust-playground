class Person:
    def __init__(self, name: str, age: int = None):
        self.name = name
        self.age = age

    @staticmethod
    def new(name: str):
        return Person(name)

    def set_age(self, age: int):
        self.age = age
        return self

    def check_age(self):
        print(
            f"{self.name} is {self.age} years old."
            if self.age
            else f"Who knows how old {self.name} is?"
        )


Person.new("Sally").set_age(27).check_age()
Person.new("Bill").check_age()
Person.new("Pedro").set_age(34).set_age(23).check_age()
