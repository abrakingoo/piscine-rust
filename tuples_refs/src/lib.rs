#[derive(Debug, PartialEq)]

// Define a tuple struct to represent a Student. Each is identified by an id of type u32, their first name and last name.
pub struct Student(pub u32, pub String, pub String)


// functions to return the id, first name and last name.
pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}