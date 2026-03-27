use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    score: f64,
    fees: u64, // is_enrolled: bool,
}

impl Student {
    fn update_fees(&mut self, fees: u64) {
        self.fees += fees;
    }
}

enum AdmissionStatus {
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
            "3" => pay_fees(&mut students),
            "4" => search_student(&students),
            "5" => {
                println!("Thanks for using this system.\nSee you next time.\nBye");
                break;
            }
            _ => std::process::exit(1),
        }
    }
}

const FEES: u64 = 100_000;

// Menu
fn menu() -> String {
    println!("\nChoose an operation to carry out. Choose either 1 - 5\n");
    println!("********** Operation Menu **********");
    println!(
        "
1. Add Student
2. View All Students
3. Check fee Balance and Pay fees
4. Search Student by Name
5. Exit"
    );

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice
}
// Get inputs
fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
// Add new Students
fn add_student(students: &mut Vec<Student>) {
    let mut age: u8 = 0;
    let mut score: f64 = 0f64;
    let name = get_input("Give Student name: ");
    let name = name.trim().to_string();
    let fees: u64 = 0;

    let last_student_id: u32;
    if students.is_empty() {
        last_student_id = 0
    } else {
        last_student_id = students[students.len() - 1].id;
    }
    let id = last_student_id + 1;
    loop {
        // Check age
        if score > 0f64 && score < 100f64 && age > 0 {
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
        let score_input = get_input("Give student Score: ");
        score = match score_input.trim().parse::<f64>() {
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
            fees,
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
        let student_grade: char = get_grade(student.score);
        let admission_status = match admission_status(student) {
            AdmissionStatus::Conditional => "Conditional",
            AdmissionStatus::FullyAdmitted => "Fully Admitted",
            AdmissionStatus::Rejected => "Rejected",
        };

        println!("Student {}:", index + 1);
        println!(
            "ID: {} - Name: {} - Age: {} - Score: {} - Grade: {} - Admission Status: {}",
            student.id, student.name, student.age, student.score, student_grade, admission_status
        );
        println!("=================================================================\n")
    }
}

// Search for Student
fn search_student(students: &Vec<Student>) {
    let student_name = get_input("Give name of the student you are searching for:")
        .trim()
        .to_lowercase();

    let searched_students: Vec<&Student> = students
        .iter()
        .filter(|st| st.name.to_lowercase().contains(&student_name))
        .collect();

    // if let Some(student) = students.iter().find(|st| st.name == student_name) {
    //     println!("Student found: {:#?}", student);
    // } else {
    //     println!("Student not found");
    // }

    if searched_students.is_empty() {
        println!("No students found matching '{}'", student_name);
        return;
    }

    for student in searched_students {
        println!("Match found: {:#?}", student);
    }
}

fn get_grade(score: f64) -> char {
    let mut grade = 'F';
    if score > 80f64 {
        grade = 'A';
    } else if score >= 70f64 {
        grade = 'B';
    } else if score > 60f64 {
        grade = 'C';
    } else if score > 40f64 {
        grade = 'D';
    }
    grade
}

fn pay_fees(students: &mut Vec<Student>) {
    let student_name = get_input("Give student name to pay fees:");

    if let Some(student) = students
        .iter_mut()
        .find(|st| st.name.to_lowercase() == student_name)
    {
        println!("\nNote that the total fees for the year is {}frs ", FEES);
        let owing = FEES - student.fees;
        println!("You are currently owing {}frs\n", owing);
        if owing <= 0 {
            println!("You are not owing any fees and you are fully enrolled");
            println!("Thanks for trusting us.\n");
            return;
        }
        loop {
            let updated_fees = get_input("How much do you wish to pay? ");
            let updated_fees: u64 = match updated_fees.parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Fees must be a positive number, no 'frs' or text should be added");
                    continue;
                }
            };
            if updated_fees > owing {
                println!("Give an amount less than or equal to {}", owing);
                continue;
            }
            student.update_fees(updated_fees);

            println!("You are now owing {}frs", owing - updated_fees);
            break;
        }
    } else {
        println!("No match found for Student with name: {} ", student_name)
    }
}

fn admission_status(student: &Student) -> AdmissionStatus {
    let mut status = AdmissionStatus::Conditional;

    if student.fees >= FEES / 2 {
        status = AdmissionStatus::FullyAdmitted
    } else if student.age < 15 {
        status = AdmissionStatus::Rejected
    }

    status
}
