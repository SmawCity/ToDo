use std::io::{stdin, stdout, Write};

struct TodoItem {
    name: String,
    priority: String,
    completed: bool,
}

fn main() 
{
    let mut todo_list = Vec::new();
    println!("To-Do List! in Rust");
    loop 
    {
        println!("\n1. Add item");
        println!("2. Complete item");
        println!("3. Show items");
        println!("4. Delete Completed Items");
        println!("5. Quit");

        let mut choice = String::new();
        print!("Enter your choice: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut choice).unwrap();

        match choice.trim() 
        {
            "1" => {
                let mut item_name = String::new();
                print!("\nEnter the name of the item: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut item_name).unwrap();

                let mut item_priority = String::new();
                print!("\nEnter the priority of the item (Low, Medium, High): ");
                stdout().flush().unwrap();
                stdin().read_line(&mut item_priority).unwrap();

                add_item(&mut todo_list, item_name.trim().to_string(), item_priority.trim().to_string());
            },
            "2" => {
                show_items(&todo_list);
                let mut index_str = String::new();
                print!("\nEnter the index of the item to complete: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut index_str).unwrap();
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    complete_item(&mut todo_list, index);
                }
            },
            "3" => {
                show_items(&todo_list);
            },
            "4" => {
                delete_completed(&mut todo_list);
            }
            "5" => break,
            _ => println!("\nInvalid choice!"),
        }
    }
}

fn add_item(todo_list: &mut Vec<TodoItem>, name: String, priority: String) 
{
    let item = TodoItem {
        name,
        priority,
        completed: false,
    };
    todo_list.push(item);
}

fn complete_item(todo_list: &mut Vec<TodoItem>, index: usize) 
{
    if index == 0 || index > todo_list.len() {
        println!("\nNo item exists at that index.");
        return;
    }
    let list_index = index - 1;
    if let Some(item) = todo_list.get_mut(list_index) {
        item.completed = true;
    }
}

fn show_items(todo_list: &Vec<TodoItem>) 
{
    for (index, item) in todo_list.iter().enumerate() {
        let status = if item.completed { "completed" } else { "not completed" };
        println!("{}: {} | [Priority: {}] | [{}]", index+1, item.name, item.priority, status);
    }
}

fn delete_completed(todo_list: &mut Vec<TodoItem>) 
{
    todo_list.retain(|item| !item.completed);
}

