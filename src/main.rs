mod hashmap_department;
use crate::hashmap_department::hashmap_department::department;
fn main() {
    let employees: Vec<&str> = vec![
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

    let departments = vec!["Tech", "Sales", "Accounting", "Cooking", "Cleaning"];
    println!("{:#?}", department(employees, departments, "Cooking"));
}