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
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

// 2. enum PersonType
#[derive(Debug, PartialEq, Eq, Hash)]
enum PersonType {
    Student,
    Teacher,
}

// 3. enum for Errors
#[derive(Debug, PartialEq)]
enum PersonManagerError {
    NotFound(String),
    AlreadyExists(String),
}

impl fmt::Display for PersonManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersonManagerError::NotFound(name) => write!(f, "Person '{}' not found.", name),
            PersonManagerError::AlreadyExists(name) => {
                write!(f, "Person '{}' already exists.", name)
            }
        }
    }
}

// 4. HashMap
#[derive(Debug)]
struct PersonManager {
    people: HashMap<PersonType, Vec<Person>>,
}

// 5. add_person, remove_person, list_person
impl PersonManager {
    fn new() -> Self {
        Self {
            people: HashMap::new(),
        }
    }

    fn add_person(
        &mut self,
        persontype: PersonType,
        person: Person,
    ) -> Result<(), PersonManagerError> {
        let people = self.people.entry(persontype).or_insert_with(Vec::new);
        if people.iter().any(|p| p.name == person.name) {
            return Err(PersonManagerError::AlreadyExists(person.name.clone()));
        }
        people.push(person);
        Ok(())
    }

    fn remove_person(
        &mut self,
        persontype: &PersonType,
        name: &str,
    ) -> Result<(), PersonManagerError> {
        if let Some(people) = self.people.get_mut(persontype) {
            if let Some(pos) = people.iter().position(|p| p.name == name) {
                people.remove(pos);
                return Ok(());
            }
        }
        Err(PersonManagerError::NotFound(name.to_string()))
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
        println!();
    }

    // 6. Display all persons
    fn display_all_person(&self) {
        println!("Display all persons");
        for (persontype, person) in &self.people {
            println!("{:?}", persontype);
            for p in person {
                println!("{}", p);
            }
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_person() {
        let mut manager = PersonManager::new();
        let person = Person::new("Alice".to_string(), 20);
        
        assert_eq!(manager.add_person(PersonType::Student, person.clone()), Ok(()));
        assert_eq!(manager.people.get(&PersonType::Student).unwrap().len(), 1);

        assert_eq!(manager.add_person(PersonType::Student, person), Err(PersonManagerError::AlreadyExists("Alice".to_string())));
    }

    #[test]
    fn test_remove_person() {
        let mut manager = PersonManager::new();
        let person = Person::new("Bob".to_string(), 25);
        
        manager.add_person(PersonType::Teacher, person.clone()).unwrap();
        
        assert_eq!(manager.remove_person(&PersonType::Teacher, "Bob"), Ok(()));
        assert!(manager.people.get(&PersonType::Teacher).unwrap().is_empty());

        assert_eq!(manager.remove_person(&PersonType::Teacher, "Bob"), Err(PersonManagerError::NotFound("Bob".to_string())));
    }

    #[test]
    fn test_list_person() {
        let mut manager = PersonManager::new();
        let person1 = Person::new("Charlie".to_string(), 30);
        let person2 = Person::new("Dana".to_string(), 28);

        manager.add_person(PersonType::Student, person1).unwrap();
        manager.add_person(PersonType::Student, person2).unwrap();

        let student_list = manager.people.get(&PersonType::Student).unwrap();
        assert_eq!(student_list.len(), 2);
    }
}


fn main() -> Result<(), PersonManagerError> {
    let p = Person::new(String::from("Python"), 18);
    let r = Person::new(String::from("Rust"), 17);
    let c = Person::new(String::from("C++"), 52);

    let mut manager = PersonManager::new();

    manager.add_person(PersonType::Student, p)?;
    manager.add_person(PersonType::Student, r)?;
    manager.add_person(PersonType::Teacher, c)?;

    manager.list_person(&PersonType::Student);
    manager.display_all_person();

    manager.remove_person(&PersonType::Student, "Python")?;
    manager.display_all_person();

    manager.remove_person(&PersonType::Student, "Java")?;

    Ok(())
}
