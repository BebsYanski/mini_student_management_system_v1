use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    score: f64,
    // is_enrolled: bool,
}

enum AdmitionStatus {
    FullyAdmitted,
    Conditional,
    Rejected,
}
fn main() {
    // variables
    let mut students: Vec<Student> = Vec::new();

    println!("\nMini Student Management System\n");

    loop {
        match menu().trim() {
            "1" => add_student(&mut students),
            "2" => view_students(&students),
            "3" => search_student(&students),
            "4" => {
                println!("Thanks for using this system.\nSee you next time.\nBye");
                break;
            }
            _ => std::process::exit(1),
        }
    }
}

// Menu
fn menu() -> String {
    println!("\nChoose an operation to carry out. Choose either 1 - 5\n");
    println!("********** Operation Menu **********");
    println!(
        "
1. Add Student
2. View All Students
3. Search Student by Name
4. Exit"
    );

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice
}
// Get inputs
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}
// Add new Students
fn add_student(students: &mut Vec<Student>) {
    let mut age: u8 = 0;
    let mut score: f64 = 0f64;
    let name = get_input("Give Student name: ");
    let name = name.trim().to_string();

    let last_student_id: u32;
    if students.is_empty() {
        last_student_id = 0
    } else {
        last_student_id = students[students.len() - 1].id;
    }
    let id = last_student_id + 1;
    loop {
        // Check age
        if score > 0f64 && age > 0 {
            break;
        }

        if age <= 0 {
            let age_input = get_input("Give Student age: ");

            age = match age_input.trim().parse() {
                Ok(result) => result,
                Err(_) => {
                    println!("Error: Age must be a number");
                    continue;
                }
            };

            if age <= 0 {
                println!("Age must be greater than 0");
                continue;
            }
        }

        // Receive Score and check
        let score = get_input("Give student Score: ");
        let score = match score.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Error: Score must be a number");
                continue;
            }
        };

        if score > 100f64 || score < 0f64 {
            println!("Score must be between 0 and 100 inclusive");
            continue;
        }

        students.push(Student {
            id,
            name,
            age,
            score,
        });
        break;
    }
}

// View Students
fn view_students(students: &Vec<Student>) {
    if students.is_empty() {
        println!("There are no students yet. You can choose 1 to add a student");
        return;
    }
    // for student in students {
    //     println!("{:#?}", student);
    // }

    // for (index, student) in students.iter().enumerate() {
    //     println!("{}: {:#?}", index + 1, student);
    // }

    for (index, student) in students.iter().enumerate() {
        println!("{}: {:#?}", index + 1, student);
    }
}

// Search for Student
fn search_student(students: &Vec<Student>) {
    todo!()
}
