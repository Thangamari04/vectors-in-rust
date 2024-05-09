// src/main.rs
mod student;

use student::{Marks, Student};

fn main() {
    let mut students: Vec<Student> = Vec::new();
    println!("STUDENT DETAILS \n");
   
    students.push(Student::new(
        String::from("Thangam"),
        String::from("12345"),
        String::from("10th standared"),
        85,
        90,
        95,
        88,
        82,
    ));
    
    

    for student in &students {
        println!(
            "Name: {}, \nReg No: {}, \nclass: {}, \nMarks: [Tamil: {}, English: {}, Maths: {}, Science: {}, Social Science: {}]",
            student.name,
            student.reg_no,
            student.class,
            student.marks.tamil,
            student.marks.english,
            student.marks.maths,
            student.marks.science,
            student.marks.social_science
        );
    }
}
