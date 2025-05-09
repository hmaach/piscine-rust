pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    student.1.as_str()
}

pub fn last_name(student: &Student) -> &str {
    student.2.as_str()
}
