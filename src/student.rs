// src/student.rs

pub struct Student {
    pub name: String,
    pub reg_no: String,
    pub class: String,
    pub marks: Marks,
}

pub struct Marks {
    pub tamil: u32,
    pub english: u32,
    pub maths: u32,
    pub science: u32,
    pub social_science: u32,
}

impl Student {
    pub fn new(
        name: String,
        reg_no: String,
        class: String,
        tamil: u32,
        english: u32,
        maths: u32,
        science: u32,
        social_science: u32,
    ) -> Student {
        let marks = Marks {
            tamil,
            english,
            maths,
            science,
            social_science,
        };
        Student {
            name,
            reg_no,
            class,
            marks,
        }
    }
}
