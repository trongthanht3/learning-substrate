use std::collections::{BTreeSet, HashMap};
use itertools::Itertools;

fn main() {
    let mut mta = School::new();
    mta.add("binh".to_string(), 2);
    mta.add("minh".to_string(), 6);
    mta.add("an".to_string(), 1);
    mta.add("long".to_string(), 1);

    println!("grades: {:?}", mta.grade(1));

    // mta.in_key();
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct School {
    students: HashMap<String, u32>
}

impl School {
    pub fn new() -> Self {
        School { students: HashMap::new() }
    }

    pub fn add(&mut self, name: String, grade: u32) {
        self.students.insert(name, grade);
    }

    pub fn grades(&self) -> Vec<&u32> {
        let grade_vec: BTreeSet<_> = self.students.values().collect();

        return grade_vec.into_iter().collect();
    }

    pub fn find_student(&self, grade: u32) -> HashMap<String, u32> {
        let grade_eq = HashMap::from_iter(self.students.iter()
                                                    .filter(|&(k,v)| *v==grade)
                                                    .map(|(k, v)| ((*k).to_string(), *v)));

        return grade_eq;        
        
    }

    pub fn grade(&self, grade: u32) -> Vec<String>{
        let grade_eq = self.find_student(grade);

        let mut grade_vec: Vec<String> = Vec::new();
        for (name, value) in &grade_eq {
            grade_vec.push((*name).to_string());
        }

        grade_vec.sort();

        return grade_vec;
    }


}