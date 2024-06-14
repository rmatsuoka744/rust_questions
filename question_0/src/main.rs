use std::collections::HashMap;
use std::fmt;

// 1. struct Person
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

impl Person {
    fn new(name: String, age:u32) -> Person {
        Person {
            name,
            age,
        }
    }
}

// 2. enum PersonType
#[derive(Debug, PartialEq, Eq, Hash)]
enum PersonType {
    Student,
    Teacher,
}

// 3. HashMap
#[derive(Debug)]
struct PersonManager {
    people: HashMap<PersonType, Vec<Person>>,
}

// 4. add_person, remove_person, list_person
impl PersonManager {
    fn new() -> Self {
        Self {
            people: HashMap::new(),
        }
    }

    fn add_person(&mut self, persontype: PersonType, person: Person) {
        self.people.entry(persontype).or_insert_with(Vec::new).push(person);
    }

    fn remove_person(&mut self, persontype: &PersonType, name: &str) -> bool {
        if let Some(people) = self.people.get_mut(persontype) {
            if let Some(pos) = people.iter().position(|p| p.name == name) {
                people.remove(pos);
                return true;
            }
        }
        false
    }

    fn list_person(&self, persontype: &PersonType) {
        if let Some(people) = self.people.get(persontype) {
            println!("Display all {:?}", persontype);
            for person in people {
                println!("{}", person);
            }
        } else {
            println!("No person found for {:?}", persontype);
        }
        println!("");
    }

// 5. Display all person
    fn display_all_person(&self) {
        println!("Display all person");
        for (persontype, person) in &self.people {
            println!("{:?}", persontype);
            for p in person {
                println!("{}", p);
            }
        }
        println!("");
    }
}


fn main() {
    let bob = Person::new(String::from("Bob"), 18);
    let alice = Person::new(String::from("Alice"), 17);
    let teacher = Person::new(String::from("Teacher"), 52);

    let mut manager = PersonManager::new();
    
    manager.add_person(PersonType::Student, bob);
    manager.add_person(PersonType::Student, alice);
    manager.add_person(PersonType::Teacher, teacher);

    manager.list_person(&PersonType::Student);
    manager.display_all_person();

    manager.remove_person(&PersonType::Student, "Bob");

    manager.display_all_person();
    
}