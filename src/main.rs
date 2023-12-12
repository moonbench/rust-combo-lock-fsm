use std::fmt::Debug;
use std::io::{self, Write};

#[derive(Debug)]
struct Lock {
    state: Option<Box<dyn State>>,
    combination: [i8; 3],
}

impl Lock {
    pub fn new(first: i8, second: i8, third: i8) -> Lock {
        Lock {
            state: Some(Box::new(UnlockedOpen { })),
            combination: [first, second, third] 
        }
    }

    fn print_status(&self) {
        if let Some(s) = self.state.as_ref() {
            println!("{}", &s.draw());
        } else {
            print_error("Invalid lock state");
            unreachable!();
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.handle_event(event, self));
        }
    }
}

trait State {
    fn name(&self) -> String;
    fn draw(&self) -> String;
    fn handle_event(self: Box<Self>, _event: Event, _lock: &mut Lock) -> Box<dyn State>;
}

struct UnlockedOpen {}
impl State for UnlockedOpen {
    fn name(&self) -> String {
        String::from("Unlocked and Open")
    }
    fn draw(&self) -> String {
        String::from(r#"
         ________
        |  ____  |
        | |    | |
        ```    | |
         ______|_|
        |   __   |
        |  [  ]  | OPEN
        |   []   | UNLOCKED
        |   []   |
         \______/"#)
    }
    fn handle_event(self: Box<Self>, event: Event, lock: &mut Lock) -> Box<dyn State> {
        match event {
            Event::Close => {
                self.close()
            },
            Event::ChangeCode(first, second, third) => {
                self.update_combination(lock, first, second, third)
            },
            _ => {
                print_error("Not allowed for an open lock. Try closing it first.");
                self
            },
        }
    }
}

impl UnlockedOpen {
    fn close(self: Box<Self>) -> Box<dyn State> {
        print_success("Lock closed");
        Box::new(UnlockedClosed {})
    }
    fn update_combination(self: Box<Self>, lock: &mut Lock, first: i8, second: i8, third: i8) -> Box<dyn State> {
        print_success("Code changed");
        lock.combination = [first, second, third];
        self
    }
}

struct UnlockedClosed {}
impl State for UnlockedClosed {
    fn name(&self) -> String {
        String::from("Unlocked and Closed")
    }
    fn draw(&self) -> String {
        String::from(r#"
         ________
        |  ____  |
        |_|____|_|
        |   __   |
        |  [  ]  | CLOSED
        |   []   | UNLOCKED
        |   []   |
         \______/"#)
    }
    fn handle_event(self: Box<Self>, event: Event, _lock: &mut Lock) -> Box<dyn State> {
        match event {
            Event::Open => {
                self.open()
            },
            Event::Lock => {
                self.lock()
            }
            _ => {
                print_error("Not allowed for an already closed and unlocked lock");
                self
            },
        }
    }
}

impl UnlockedClosed {
    fn lock(self: Box<Self>) -> Box<dyn State> {
        print_success("Locked");
        Box::new(Locked {})
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        print_success("Lock opened");
        Box::new(UnlockedOpen {})
    }
}

struct Locked {}
impl State for Locked {
    fn name(&self) -> String {
        String::from("Locked")
    }
    fn draw(&self) -> String {
        String::from(r#"
         ________
        |  ____  |
        |_|____|_|
        |   __   |
        |  [**]  | CLOSED
        |   []   | LOCKED
        |   []   |
         \______/"#)
    }
    fn handle_event(self: Box<Self>, event: Event, lock: &mut Lock) -> Box<dyn State> {
        match event {
            Event::Unlock(first, second, third) => {
                self.unlock(first, second, third, lock.combination)
            },
            _ => {
                print_error("Not allowed for a locked lock. Unlock it first.");
                self
            },
        }
    }
}

impl Locked {
    fn unlock(self: Box<Self>, first: i8, second: i8, third: i8, combination: [i8; 3]) -> Box<dyn State> {
        if [first, second, third] == combination{
            print_success("Valid combination, unlocked!");
            Box::new(UnlockedClosed {})
        } else {
            print_error("Invalid combination, still locked");
            self
        }
    }
}

impl Debug for dyn State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}


enum Event {
    Lock,
    Unlock(i8, i8, i8),
    Open,
    Close,
    ChangeCode(i8, i8, i8),
}

fn main() {
    println!("Finite State Machine Lock Demo\n");

    let mut lock = Lock::new(0, 0, 0);


    lock.print_status();

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
    print_info(r#"Lock State Machine Demo Help Info

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