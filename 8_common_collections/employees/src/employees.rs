use regex::Regex;
use std::collections::HashMap;

pub struct EmployeeDB {
    db: HashMap<String, Vec<String>>,
}

impl EmployeeDB {
    pub fn new() -> EmployeeDB {
        EmployeeDB { db: HashMap::new() }
    }

    pub fn list(&mut self) {
        for (k, v) in &mut self.db {
            v.sort();
            println!("{}", k);
            for name in v {
                println!("\t{}", name);
            }
        }
        println!()
    }

    pub fn insert(&mut self, name: &str, dept: &str) {
        self.db
            .entry(dept.to_string())
            .or_insert(Vec::new())
            .push(name.to_string());
    }
}

pub fn parse_employee_data(input: &str) -> Result<(String, String), &str> {
    let re = Regex::new(r"^add (.+) to (.+)$").unwrap();
    let captures = match re.captures(&input) {
        Some(caps) => caps,
        None => return Result::Err("Unable to parse employee data"),
    };

    let name = captures.get(1).unwrap().as_str().to_string();
    let dept = captures.get(2).unwrap().as_str().to_string();
    Result::Ok((name, dept))
}
