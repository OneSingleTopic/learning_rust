use std::collections::HashMap;
use std::io;

enum Fonction {
    Add(String, String),
    ListDepartement(String),
    ListAll,
    Quit,
    None,
}

struct Company {
    departements: HashMap<String, Vec<String>>,
}
impl Company {
    fn new() -> Company {
        Company {
            departements: HashMap::new(),
        }
    }
    fn add_employee(&mut self, employee_name: String, department: String) {
        println!("Adding {} to {} department", employee_name, department);
        let department_staff = self.departements.entry(department).or_insert(Vec::new());
        department_staff.push(employee_name);
    }
    fn list_department(&self, department: &String) {
        let department_staff = self.departements.get(department);
        match department_staff {
            Some(employees) => {
                println!("{} department :", department);
                for employee in employees {
                    println!("- {}", employee);
                }
            }
            None => println!("{} department does not exists", department),
        }
    }
    fn list_company(&self) {
        println!("");
        println!("=======================");
        for (department, _) in &self.departements {
            self.list_department(department);
            println!("");
        }
        println!("=======================");
    }
}

fn main() {
    let mut company_departments = Company::new();
    println!("- ADD <name> in <departement>");
    println!("- List <departement>");
    println!("- List All ");
    println!("- Quit ");
    loop {
        let company_input = parse_input();
        match company_input {
            Fonction::Add(employee_name, department) => {
                company_departments.add_employee(employee_name, department)
            }
            Fonction::ListDepartement(department) => {
                company_departments.list_department(&department)
            }
            Fonction::ListAll => company_departments.list_company(),
            Fonction::Quit => {
                println!("Quiting !");
                break;
            }
            _ => {
                println!("Unknown order !");
                continue;
            }
        }
    }
}

fn parse_input() -> Fonction {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");

    parse_order(input.trim().to_string())
}

fn parse_order(order: String) -> Fonction {
    let mut words = order.split_ascii_whitespace();
    match words.next() {
        Some("Add") => {
            let word_list: Vec<_> = words.collect();
            match (word_list.get(0), word_list.get(2)) {
                (Some(name), Some(departement)) => {
                    Fonction::Add(name.to_string(), departement.to_string())
                }
                _ => Fonction::None,
            }
        }
        Some("List") => match words.next() {
            Some("All") => Fonction::ListAll,
            Some(department) => Fonction::ListDepartement(department.to_string()),
            None => Fonction::None,
        },
        Some("Quit") => Fonction::Quit,
        _ => Fonction::None,
    }
}
