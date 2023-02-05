pub mod hashmap_department {
    use rand::{thread_rng, Rng};
    use std::{collections::HashMap, io};
    pub fn department(employees: Vec<&str>, departments: Vec<&str>, depar: &str) -> Vec<(String, String)> {

        let mut map: HashMap<String, String> = HashMap::new();
        for employee in &employees {
            map.insert(
                employee.to_string(),
                departments[thread_rng().gen_range(0..departments.len())].to_string(),
            );
        }
    
        let tech_filter: HashMap<String, String> = map.into_iter().filter(|(_employee, department)| department == depar).collect();
        let mut tech_sort: Vec<(String, String)> = tech_filter.into_iter().collect();
        tech_sort.sort();
        tech_sort
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
    
}