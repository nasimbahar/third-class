pub fn starting_point(){
    BasicExample();
   
   
}

struct Student {
    name: String,
    grade: char,
}

fn BasicExample() {
    // Creating a vector of Student structs
    let students = vec![
        Student { name: "Alice".to_string(), grade: 'A' },
        Student { name: "Bob".to_string(), grade: 'B' },
    ];

    // Iterating over the vector
    for student in &students {
        println!("{} received grade {}", student.name, student.grade);
    }
}
