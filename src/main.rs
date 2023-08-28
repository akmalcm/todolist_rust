use chrono::{ DateTime, Local };
use std::fmt::Write;

struct Todo {
    id: usize,
    title: String,
    description: String,
    created_at: DateTime<Local>
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Todo {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            created_at: self.created_at
        }
    }
}

fn main() {
    let mut list: Vec<Todo> = Vec::new();
    let mut prompt = true;

    while prompt {
        println!("1. Add todo");
        println!("2. Edit todo");
        println!("3. Delete todo");
        println!("4. Print todos");
        println!("5. Exit");
        println!("Enter your choice :");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let trimmed = choice.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                match i {
                    1 => {
                        let id: usize = list.last().map_or(0, |todo| todo.id) + 1;
                        let todo = prompt_todo(id);
                        add_todo(&mut list, todo);
                    },
                    2 => {
                        println!("Enter id to edit :");
                        let mut input: String = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");

                        let id = input.trim().parse::<usize>().expect("This was not an integer");
                        if list.iter().position(|value| value.id == id).is_none() {
                            println!("ID not found");
                        }else{
                            let todo = prompt_todo(id);
                            edit_todo(&mut list, todo)
                        }

                    },
                    3 => {
                        println!("Enter id to delete :");
                        let mut input: String = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        let id = input.trim().parse::<usize>().expect("This was not an integer");

                        // if id not present in list then return
                        if list.iter().position(|value| value.id == id).is_none() {
                            println!("ID not found");
                        }else{
                            delete_todo(&mut list, id)
                        }
                    },
                    4 => {
                        print_todos(&list);
                    },
                    5 => {
                        println!("Thanks for using this todo list app");
                        prompt = false;
                    },
                    _ => {
                        println!("Invalid choice");
                    }
                }
            },
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
    }
}

fn add_todo(list: &mut Vec<Todo>, todo: Todo){
    list.push(todo);
}

fn delete_todo(list: &mut Vec<Todo>, id: usize){
    // Split the list in 2
    let mut split_list = list.split_off(id-1);
    // Remove the first element of the second half
    split_list.remove(0);
    // Join the 2 halves back together
    list.append(&mut split_list);

    println!("Todo with id {} deleted", id);
}

fn edit_todo(list: &mut Vec<Todo>, todo: Todo){
    let index_in_list = list.iter().position(|value| value.id == todo.id);

    if index_in_list.is_none() {
        return;
    }
    list[index_in_list.unwrap()] = todo;

    println!("Todo with id {} updated", list[index_in_list.unwrap()].id);
}

fn print_todos(list: &Vec<Todo>) {
    let mut formatted_str = String::new();
    formatted_str.push('[');

    for value in list.iter() {
        write!(
            formatted_str,
            "{{ {}, {}, {}, {} }}",
            value.id, value.title, value.description, value.created_at
        )
        .expect("Failed to write to buffer");

        if value.id != list.last().unwrap().id {
            write!(formatted_str, ", ").expect("Failed to write to buffer");
        }
    }

    formatted_str.push(']');

    println!("{}", formatted_str);
}

fn prompt_todo(id: usize) -> Todo {
    let mut input: String = String::new();

    input.clear();
    println!("Enter title :");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let title = input.clone();

    input.clear();
    println!("Enter description :");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let description = input.clone();

    let new_todo = Todo {
        id: id,
        title: String::from(title.trim()),
        description: String::from(description.trim()),
        created_at: Local::now()
    };

    return new_todo;
}
