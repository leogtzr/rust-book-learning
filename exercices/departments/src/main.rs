use std::collections::HashMap;
use std::io;

fn main() {
    let mut company = HashMap::new();

    loop {
        println!("Enter a command (add <name> to <department>, list <department>, or list all):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.starts_with("add") {
            let parts: Vec<_> = input.split_whitespace().collect();
            if parts.len() != 4 {
                println!("Invalid command format");
                continue;
            }
            let name = parts[1].to_uppercase().trim().to_string();
            let department = parts[3].to_string();
            company.entry(department).or_insert_with(Vec::new).push(name);
        } else if input.starts_with("list") {
            let parts: Vec<_> = input.split_whitespace().collect();
            if parts.len() != 2 {
                println!("Invalid command format");
                continue;
            }
            if parts[1] == "all" {
                for (department, employees) in &company {
                    println!("{}:", department);
                    let mut sorted_employees: Vec<_> = employees.iter().cloned().collect();
                    sorted_employees.sort_unstable();
                    for employee in sorted_employees {
                        println!("- {}", employee);
                    }
                }
            } else {
                let department = parts[1].to_string();
                if let Some(employees) = company.get(&department) {
                    println!("{}:", department);
                    let mut sorted_employees: Vec<_> = employees.iter().cloned().collect();
                    sorted_employees.sort_unstable();
                    for employee in sorted_employees {
                        println!("- {}", employee);
                    }
                } else {
                    println!("Department not found");
                }
            }
        } else {
            println!("Invalid command");
        }
    }
}