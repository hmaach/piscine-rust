use tuples_refs::*;

fn main() {
    let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
    println!("Student's first name: {}", first_name(&student));
    println!("Student's last name: {}", last_name(&student));
    println!("Student's id: {}", id(&student));
}
