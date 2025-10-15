use std::io;

enum Status {
    Todo,
    InProgress,
    Done,
}

struct Task {
    description: String,
    status: Status,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    println!("Welcome to the To-Do List Manager!");
    loop {
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Update Task Status");
        println!("4. Exit\n");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let mut description = String::new();
                println!("Enter task description:");
                io::stdin().read_line(&mut description).expect("Failed to read line");
                tasks.push(Task {
                    description: description.trim().to_string(),
                    status: Status::Todo,
                });
            }
            "2" => {
                for (i, task) in tasks.iter().enumerate() {
                    let status = match task.status {
                        Status::Todo => "Todo",
                        Status::InProgress => "In Progress",
                        Status::Done => "Done",
                    };
                    println!("{}: {} [{}]", i + 1, task.description, status);
                }
            }
            "3" => {
                let mut index_str = String::new();
                println!("Enter task number to update:");
                io::stdin().read_line(&mut index_str).expect("Failed to read line");
                let index: usize = match index_str.trim().parse::<usize>() {
                    Ok(num) => {
                        if num == 0 {
                            println!("Please enter a valid task number!");
                            continue;
                        }
                        num - 1
                    }
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
                if index >= tasks.len() {
                    println!("Invalid task number!");
                    continue;
                }

                println!("Select new status:");
                println!("1. Todo");
                println!("2. In Progress");
                println!("3. Done");

                let mut status_choice = String::new();
                io::stdin().read_line(&mut status_choice).expect("Failed to read line");

                let newstatus = match status_choice.trim() {
                    "1" => Status::Todo,
                    "2" => Status::InProgress,
                    "3" => Status::Done,
                    _ => {
                        println!("Invalid status choice!");
                        continue;
                    }
                };

                tasks[index].status = newstatus;
                println!("Task updated successfully!");
            }
            "4" => break,
            _ => println!("Invalid choice!"),
        }
    }
}