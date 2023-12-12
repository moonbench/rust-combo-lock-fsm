use std::io::{self, Write};
use crate::combo_lock::{Lock, Event};

mod combo_lock;

fn main() {
    println!("Finite State Machine Lock Demo\n");

    let mut lock = Lock::new(0, 0, 0);

    lock.print_status();
    println!("");

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap(); // Place the user's input on the same line as the prompt

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let line: Vec<&str> = buffer.trim().split_whitespace().collect();

        if let Some(command) = line.get(0) {
            match command.to_owned() {
                "new" => {
                    lock = new_lock_from_line(line);
                    lock.print_status();
                },
                "set" => {
                    set_combination_from_line(&mut lock, line);
                    lock.print_status();
                }
                "open" => {
                    lock.handle_event(Event::Open);
                    lock.print_status();
                },
                "close" => {
                    lock.handle_event(Event::Close);
                    lock.print_status();
                },
                "lock" => {
                    lock.handle_event(Event::Lock);
                    lock.print_status();
                },
                "unlock" => {
                    unlock_from_line(&mut lock, line);
                    lock.print_status();
                },
                "info" | "status" => lock.print_status(),
                "debug" => println!("{:#?}", lock),
                "exit" | "quit" => break,
                "h" | "help" | "?" => print_commands(),
                _ => print_error(&format!("Unknown Command ({}) ('help' for help)", buffer.trim())),
            }
        }
        println!("");
    }
}

fn new_lock_from_line(line: Vec<&str>) -> Lock {
    if let Ok(values) = parse_combo(line) {
        print_info("Creating a new lock using provided combination (* * *)...");
        Lock::new(values[0], values[1], values[2])
    } else {
        print_info("Creating a new lock using default combination (0 0 0)...");
        Lock::new(0,0,0)
    }
}

fn set_combination_from_line(lock: &mut Lock, line: Vec<&str>) {
    let parsed = parse_combo(line);
    if let Ok(values) = parsed {
        lock.handle_event(Event::ChangeCode(values[0], values[1], values[2]));
    } else if let Err(msg) = parsed {
        print_error(&msg);
    }
}

fn unlock_from_line(lock: &mut Lock, line: Vec<&str>) {
    let parsed = parse_combo(line);
    if let Ok(values) = parsed {
        lock.handle_event(Event::Unlock(values[0], values[1], values[2]));
    } else if let Err(msg) = parsed {
        print_error(&msg);
    }
}

fn parse_combo(line: Vec<&str>) -> Result<[i8; 3], String> { 
    let first = line.get(1);
    let second = line.get(2);
    let third = line.get(3);
    if let (Some(first), Some(second), Some(third)) = (first, second, third) {
        let first = first.parse::<i8>();
        let second = second.parse::<i8>();
        let third = third.parse::<i8>();
        
        if let (Ok(first), Ok(second), Ok(third)) = (first, second, third) {
            Ok([first, second, third])
        } else {
            Err("All values must be numeric".to_owned())
        }
    } else {
        Err("Three numeric values required".to_owned())
    }
}

fn print_commands() {
    println!(r#"Lock State Machine Demo Help Info

This demo program simulates a mechanical lock via a finite state machine implemented in Rust.

State Diagram:
   ______________                ________________                 ________
  |              | -- CLOSE --> |                | --- LOCK ---> |        |
  | UnlockedOpen |              | UnlockedClosed |               | Locked |
  |______________| <-- OPEN --- |________________| <-- UNLOCK -- |________|

Possible Commands:

    new [<number> <number> <number>]
        Creates a new lock. Will use the specified code if provide, otherwise defaults to 0 0 0.

    set <number> <number> <number>
        Changes the lock's code to the values provided. Lock must already be unlocked and open.
    
    open
        Opens the lock's shackle. Lock must already be unlocked.
    
    close
        Closes the lock's shackle.
    
    lock
        Secures the lock. Lock must already be closed.
    
    unlock <number> <number> <number>
        Unsecures the lock if the provided numbers match the lock's combination. Lock must already be locked.
    
    info, status
        Prints information about the lock's state.
    
    debug
        Prints extended information about the lock and its state.
    
    quit, exit
        Exits the program.
    
    help, h, ?
        Displays this help information."#);
}

fn print_info(message: &str) {
    println!("[INFO] {}", message);
}

fn print_error(error: &str) {
    println!("[ERROR] {}", error);
}

fn print_success(error: &str) {
    println!("[DONE] {}", error);
}