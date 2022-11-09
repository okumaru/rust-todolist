use std::io;
use std::fs;
use std::fs::File;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

fn main() {
    println!("\n### Lets manage your todolist! ### \n");

    let mut todolist = ToDoList::new();
    todolist.help();

    loop {
        println!("---");
        println!("\nWhat will you do today?");

        let mut input = String::new();

        // Request terminal to input some words.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        // Remove new line after user do enter on terminal and set to be string.
        let input: String = input
            .trim()
            .to_string();

        // Do next if user do enter on terminal.
        if input.is_empty() { continue; }

        // Split written words into action and item name.
        let (action, item_name): (ToDoActions, String) = todolist.read_input(input);

        match action {
            ToDoActions::Add => todolist.add(&item_name),
            ToDoActions::Process => todolist.process(&item_name),
            ToDoActions::Pause => todolist.pause(&item_name),
            ToDoActions::Done => todolist.done(&item_name),
            ToDoActions::Remove => todolist.remove(&item_name),
            ToDoActions::Status => todolist.status(),
            ToDoActions::Help => todolist.help(),
            ToDoActions::Exit => {
                println!("You closing application!");
                println!("Your todolist has been saved, you can open it again another time.");
                break;
            },
            _ => println!("Does not found action!,\nplease use `help` to show list of actions command."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ToDoList {
    list: Vec<ToDo>,
}

#[allow(unused_must_use)]
impl ToDoList {
    fn new() -> ToDoList {
        let dbfile = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")
            .expect("Failed to get dbfile");

        match serde_json::from_reader::<File, Vec<ToDo>>(dbfile) {
            
            // File json decoded successfully
            Ok(todo) => ToDoList { list: todo },
            
            // File json unsuccess to decoded
            Err(e) if e.is_eof() => ToDoList { list: vec![] },
            
            // Other error
            _ => panic!("error while reading json"),
        }
    }

    fn help(&self) {
        for action in ToDoActions::iter() {
            match action {
                ToDoActions::Add => println!("{: <15} {}", String::from("add: <foo>"), String::from("add the <foo> to the list of todolist")),
                ToDoActions::Process => println!("{: <15} {}", String::from("process: <foo>"), String::from("update todolist <foo> to 'progress'")),
                ToDoActions::Pause => println!("{: <15} {}", String::from("pause: <foo>"), String::from("update todolist <foo> to 'paused'")),
                ToDoActions::Done => println!("{: <15} {}", String::from("done: <foo>"), String::from("update todolist <foo> to 'finish'")),
                ToDoActions::Remove => println!("{: <15} {}", String::from("remove: <foo>"), String::from("delete todolist <foo> from todolist")),
                ToDoActions::Status => println!("{: <15} {}", String::from("status"), String::from("show your todolist")),
                ToDoActions::Help => println!("{: <15} {}", String::from("help"), String::from("show command of application")),
                ToDoActions::Exit => println!("{: <15} {}", String::from("exit"), String::from("the way you exit the application")),
                _ => println!("{: <15}", String::from(""))
            }
        }
    }

    fn read_input(&self, input: String) -> (ToDoActions, String) {
        let action_str: String = input
            .split(":")
            .next()
            .unwrap()
            .to_lowercase();

        // Get length of first word.
        let action_len = if input.len() > action_str.len() { &action_str.len() + 2 } else { 0 };

        let action = match action_str.as_str() {
            "add" => ToDoActions::Add,
            "process" => ToDoActions::Process,
            "pause" => ToDoActions::Pause,
            "done" => ToDoActions::Done,
            "remove" => ToDoActions::Remove,
            "status" => ToDoActions::Status,
            "help" => ToDoActions::Help,
            "exit" => ToDoActions::Exit,
            _ => ToDoActions::Unknown,
        };

        // Get words next first word.
        let item_name = if input.len() > action_str.len() + 1 { 
                (&input[action_len..input.len()]).to_string()
            } else { 
                "".to_string()
            };

        (action, item_name)
    }

    fn save(&self) {
        let dbfile = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .truncate(true)
            .open("db.json")
            .expect("Failed to get dbfile");

        dbg!(&self.list);

        serde_json::to_writer_pretty(dbfile, &self.list.as_slice());
    }

    fn add(&mut self, item_name: &str) {
        self.list.push( ToDo {
            name: String::from(item_name),
            status: String::from("waiting!")
        } );

        println!("Add '{}' to todolist", item_name);

        &self.save();
    }

    fn process(&mut self, item_name: &str) {
        for item in &mut self.list {
            if item.name == item_name {
                item.status = String::from("progress");
            }
        }

        println!("Processing todolist '{}'", item_name);

        &self.save();
    }

    fn pause(&mut self, item_name: &str) {
        for item in &mut self.list {
            if item.name == item_name {
                item.status = String::from("pausing!");
            }
        }

        println!("Pausing '{}' from todolist", item_name);

        &self.save();
    }

    fn done(&mut self, item_name: &str) {
        for item in &mut self.list {
            if item.name == item_name {
                item.status = String::from("complete");
            }
        }

        println!("Set todolist '{}' to be 'Done'", item_name);

        &self.save();
    }

    fn remove(&mut self, item_name: &str) {
        let mut new_list: Vec<ToDo> = Vec::new();
        for item in &self.list {
            if item.name != item_name {
                let new_item = ToDo {
                    name: String::from(&item.name), 
                    status: String::from(&item.status)
                };
                new_list.push(new_item);
            }
        }

        // self.list = vec![];

        // let dbfile = fs::OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .read(true)
        //     .truncate(true)
        //     .open("db.json")
        //     .expect("Failed to get dbfile");

        // serde_json::to_writer_pretty(dbfile, &vec![ToDo { name: String::from("asd"), status: String::from("status")}]);

        // &self.save();

        self.list = new_list;

        println!("Remove todolist '{}' from todolist", item_name);

        &self.save();
    }

    fn status(&self) {
        for item in &self.list {
            println!("{:<20} {}", item.status, item.name);
        }
    }
}

#[derive(Debug, EnumIter)]
enum ToDoActions {
    Add,
    Process,
    Pause,
    Done,
    Remove,
    Status,
    Help,
    Exit,
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
struct ToDo {
    name: String,
    status: String,
}