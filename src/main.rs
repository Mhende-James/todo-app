// Import necessary libraries for serialization and file I/O
use serde::{Deserialize, Serialize}; // For serialization & deserialization
use std::fs;                        // For file system operations
use std::io:: Write;         // For writing to files
use std::env;                       // For accessing command line arguments

// Define the structure for a Todo item
#[derive(Debug, Serialize, Deserialize)] // Enable serialization with serde
struct Todo {
    task: String,       // The Todo task description
    completed: bool,    // Completion status of the task
}

// Implement methods for the Todo struct
impl Todo {
    // Constructor to create a new Todo
    fn new(task: &str) -> Todo {
        Todo {
            task: task.to_string(), // Convert the input to a String
            completed: false,       // Set completed status to false by default
        }
    }
}

// Load todos from a JSON file
fn load_todos() -> Vec<Todo> {
    // Attempt to read the file, return an empty list if it doesn't exist
    let file_content = fs::read_to_string("todos.json").unwrap_or_else(|_| String::from("[]"));
    // Deserialize the JSON content into a Vec<Todo>
    serde_json::from_str(&file_content).unwrap_or_else(|_| vec![]) // Return an empty vector on error
}

// Save todos to a JSON file
fn save_todos(todos: &[Todo]) {
    // Serialize the Vec<Todo> into a JSON string
    let serialized = serde_json::to_string(todos).expect("Failed to serialize todos");
    // Create (or overwrite) the file and write the JSON data
    let mut file = fs::File::create("todos.json").expect("Unable to create file");
    file.write_all(serialized.as_bytes()).expect("Unable to write data");
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments into a vector
    let mut todos = load_todos(); // Load existing todos from the JSON file

    // Check if sufficient arguments were provided
    if args.len() < 2 {
        println!("Usage: todo <command> [task]"); // Usage instructions if no command is given
        return;
    }

    // Handle different commands based on user input
    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task to add."); // Error message if no task is provided
                return;
            }
            let task = &args[2]; // Get the task description from command line
            todos.push(Todo::new(task)); // Create and add the new Todo
            save_todos(&todos); // Save updated todos to the JSON file
            println!("Added task: {}", task); // Confirmation message
        },
        "list" => {
            // List all tasks
            for (i, todo) in todos.iter().enumerate() {
                let status = if todo.completed { "[x]" } else { "[ ]" }; // Display completion status
                println!("{}: {} {}", i + 1, status, todo.task); // Print each task with its status
            }
        },
        "complete" => {
            if args.len() < 3 {
                println!("Please provide the task number to complete."); // Error for missing task number
                return;
            }
            let index: usize = args[2].parse().expect("Invalid index"); // Parse task number
            if index > 0 && index <= todos.len() {
                todos[index - 1].completed = true; // Mark the task as completed
                save_todos(&todos); // Save changes to the file
                println!("Marked task {} as completed.", index); // Confirmation message
            } else {
                println!("Task number does not exist."); // Error for invalid task number
            }
        },
        _ => {
            println!("Unknown command. Use 'add', 'list', or 'complete'."); // Error for unknown commands
        }
    }
}
