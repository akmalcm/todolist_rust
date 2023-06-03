use chrono::{ DateTime, Utc };
use std::collections::LinkedList;
use std::fmt::Write;

struct Todo {
    id: usize,
    title: String,
    description: String,
    created_at: DateTime<Utc>
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
    let mut list: LinkedList<Todo> = LinkedList::new();

    let new_todo = Todo {
        id: 1,
        title: String::from("title 1"),
        description: String::from("description 1"),
        created_at: Utc::now()
    };
    let new_todo2 = Todo {
        id: 2,
        title: String::from("title 2"),
        description: String::from("description 2"),
        created_at: Utc::now()
    };
    let del_todo = new_todo.clone();
    let mut ed_todo = new_todo2.clone();

    add_todo(&mut list, new_todo);
    add_todo(&mut list, new_todo2);

    print_todos(&list);

    delete_todo(&mut list, del_todo.id);

    ed_todo.description = String::from("description Updated");
    edit_todo(&mut list, ed_todo);

    print_todos(&list);
}

fn add_todo(list: &mut LinkedList<Todo>, todo: Todo){
    list.push_back(todo)
}

fn delete_todo(list: &mut LinkedList<Todo>, id: usize){
    if id.le(&0){
        return;
    }
    // Split the list in 2
    let mut split_list = list.split_off(id-1);
    // Remove the first element of the second half
    split_list.pop_front();
    // Join the 2 halves back together, except for the middle element
    list.append(&mut split_list);
}

fn edit_todo(list: &mut LinkedList<Todo>, todo: Todo){
    // my old code
    // let mut index_in_list: Option<usize> = None;

    // // Split the list in 2
    // for (index, value) in list.iter().enumerate() {
    //     if value.id == todo.id {
    //         index_in_list = Some(index);
    //         break;
    //     }
    // }

    // if index_in_list == None {
    //     return
    // }

    // // Split the list in 2
    // let mut split_list = list.split_off(index_in_list.unwrap());
    // // Remove the first element of the second half
    // split_list.pop_front();
    // split_list.push_front(todo);
    // // Join the 2 halves back together, except for the middle element
    // list.append(&mut split_list);

    // optimized by chatgpt
    let index_in_list = list.iter().position(|value| value.id == todo.id);

    if index_in_list.is_none() {
        return;
    }

    if let Some(index) = index_in_list {
        let mut temp_list = LinkedList::new();

        for (i, value) in list.iter().enumerate() {
            if i != index {
                temp_list.push_back(value.clone());
            }
        }

        list.clear();
        list.push_front(todo);
        list.append(&mut temp_list);
    }
}

fn print_todos(list: &LinkedList<Todo>) {
    // my old code
    // for (index, value) in list.iter().enumerate() {
    //     if index == 0 {
    //         print!("[")
    //     }

    //     print!("{{{},{},{},{}}}", value.id, value.title, value.description, value.created_at);

    //     if index == list.len() - 1 {
    //         print!("]")
    //     } else{
    //         print!(",")
    //     }
    //     println!()
    // }

    // optimized by chatgpt
    let mut formatted_str = String::new();
    let mut is_first = true;

    for value in list.iter() {
        if is_first {
            formatted_str.push('[');
            is_first = false;
        } else {
            formatted_str.push(',');
            formatted_str.push(' ');
        }

        write!(
            formatted_str,
            "{{ {}, {}, {}, {} }}",
            value.id, value.title, value.description, value.created_at
        )
        .expect("Failed to write to buffer");
    }

    if !is_first {
        formatted_str.push(']');
    }

    println!("{}", formatted_str);
}
