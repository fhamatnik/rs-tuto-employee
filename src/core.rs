use std::collections::HashMap;

#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub department: String,
}

pub struct EmployeeDB(pub HashMap<String, Vec<String>>);

impl EmployeeDB {
    pub fn new() -> EmployeeDB {
        let h: HashMap<String, Vec<String>> = HashMap::new();
        EmployeeDB(h)
    }
    pub fn add(&mut self, employee: &Employee) {
        let (name, department) = (employee.name.to_owned(), employee.department.to_owned());
        let employees = self.0.entry(department.to_owned()).or_insert(Vec::new());
        employees.push(name.to_owned());
        employees.sort();
        println!("Added {:?}", employee);
    }

    pub fn list_one(&self, department: &String) {
        match self.0.get(department) {
            Some(list) => println!("{:?}", list),
            None => println!("There is no employees in this department."),
        }
    }

    pub fn list_all(&self) {
        if &self.0.len() == &0usize {
            println!("There is no employees in this company.");
            return;
        }
        for (department, employees) in &self.0 {
            println!("{}\n{:?}", department, employees);
        }
    }
}
