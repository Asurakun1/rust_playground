use rand::{thread_rng, Rng};
use std::{collections::HashMap, io, vec};
fn main() {
    let mut _employees: Vec<&str> = vec![
        "Hannah",
        "Ethan",
        "Emma",
        "Liam",
        "Olivia",
        "Alexander",
        "Isabella",
        "William",
        "Sophia",
        "James",
    ];

    let _departments = vec!["Tech", "Sales", "Accounting", "Cooking", "Cleaning"];
    let mut employee_map: HashMap<String, String> = HashMap::new();

    for employee in &_employees {
        employee_map.insert(
            employee.to_string(),
            _departments[thread_rng().gen_range(0.._departments.len())].to_string(),
        );
    }

    let tech_filter: &HashMap<String, String> = &employee_map.into_iter().filter(|(_employee, department)| *department == "Tech").collect();
    println!("{:#?}", tech_filter);
}

fn _add_employee() -> String {
    println!("Add employee: ");
    let mut input = String::from("");
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}

fn _add_department() -> String {
    println!("to department: ");
    let mut input = String::from("");
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}
