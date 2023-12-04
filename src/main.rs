use std::fmt::Debug;
use std::io::{self, Write};

#[derive(Debug)]
struct Lock {
    state: Option<Box<dyn State>>,
    combination: [i8; 3],
}

impl Lock {
    pub fn new(first: i8, second: i8, third: i8) -> Lock {
        print_info("Creating a new lock");
        Lock {
            state: Some(Box::new(UnlockedOpen { })),
            combination: [first, second, third] 
        }
    }

    pub fn lock(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.lock());
        }
    }

    pub fn unlock(&mut self, first: i8, second: i8, third: i8) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.unlock(first, second, third, self.combination));
        }
    }

    pub fn close(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.close());
        }
    }

    pub fn open(&mut self) {
        println!("{:?}", self.state);

        if let Some(s) = self.state.take() {
            self.state = Some(s.open());
        }
    }

    pub fn update_combination(&mut self, first: i8, second: i8, third: i8) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.update_combination(self, first, second, third));
        }
    }

    fn print_status(&self) {
        if let Some(s) = self.state.as_ref() {
            print_info(&s.status_string());
        } else {
            print_error("Invalid lock state");
            unreachable!();
        }
    }
}

trait State {
    fn name(&self) -> String;
    fn status_string(&self) -> String;
    fn lock(self: Box<Self>) -> Box<dyn State>;
    fn unlock(self: Box<Self>, first: i8, second: i8, third: i8, combination: [i8; 3]) -> Box<dyn State>;
    fn close(self: Box<Self>) -> Box<dyn State>;
    fn open(self: Box<Self>) -> Box<dyn State>;
    fn update_combination(self: Box<Self>, _lock: &mut Lock, _first: i8, _second: i8, _third: i8) -> Box<dyn State>;
}

struct UnlockedOpen {}
impl State for UnlockedOpen {
    fn name(&self) -> String {
        "UnlockedOpen".to_owned()
    }
    fn status_string(&self) -> String {
        "[Unlocked] [Open]".to_owned()
    }
    fn lock(self: Box<Self>) -> Box<dyn State> {
        print_error("Must close before locking");
        self
    }
    fn unlock(self: Box<Self>, _first: i8, _second: i8, _third: i8, _combination: [i8; 3]) -> Box<dyn State> {
        print_error("Already unlocked");
        self
    }
    fn close(self: Box<Self>) -> Box<dyn State> {
        print_info("Lock closed");
        Box::new(UnlockedClosed {})
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        print_error("Already open");
        self
    }
    fn update_combination(self: Box<Self>, lock: &mut Lock, first: i8, second: i8, third: i8) -> Box<dyn State> {
        print_info("Code changed");
        lock.combination = [first, second, third];
        self
    }
}

struct UnlockedClosed {}
impl State for UnlockedClosed {
    fn name(&self) -> String {
        "UnlockedClosed".to_owned()
    }
    fn status_string(&self) -> String {
        "[Unlocked] [Closed]".to_owned()
    }
    fn lock(self: Box<Self>) -> Box<dyn State> {
        print_info("Locked");
        Box::new(Locked {})
    }
    fn unlock(self: Box<Self>, _first: i8, _second: i8, _third: i8, _combination: [i8; 3]) -> Box<dyn State> {
        print_error("Already unlocked");
        self
    }
    fn close(self: Box<Self>) -> Box<dyn State> {
        print_error("Already closed");
        self
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        print_info("Lock opened");
        Box::new(UnlockedOpen {})
    }
    fn update_combination(self: Box<Self>, _lock: &mut Lock, _first: i8, _second: i8, _third: i8) -> Box<dyn State> {
        print_error("Must be open to change the code");
        self
    }
}

struct Locked {}
impl State for Locked {
    fn name(&self) -> String {
        "Locked".to_owned()
    }
    fn status_string(&self) -> String {
        "[Locked] [Closed]".to_owned()
    }
    fn lock(self: Box<Self>) -> Box<dyn State> {
        print_error("Lock already locked");
        self
    }
    fn unlock(self: Box<Self>, first: i8, second: i8, third: i8, combination: [i8; 3]) -> Box<dyn State> {
        if [first, second, third] == combination{
            print_info("Valid combination, unlocked!");
            Box::new(UnlockedClosed {})
        } else {
            print_error("Invalid combination, still locked");
            self
        }
    }
    fn close(self: Box<Self>) -> Box<dyn State> {
        print_error("Lock already closed (and locked)");
        self
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        print_error("Unable to open a locked lock");
        self
    }
    fn update_combination(self: Box<Self>, _lock: &mut Lock, _first: i8, _second: i8, _third: i8) -> Box<dyn State> {
        print_error("Lock must be open and unlocked to change the code");
        self
    }
}


impl Debug for dyn State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn main() {
    let mut lock = Lock::new(0, 0, 0);

    println!("Finite State Machine Lock Demo");
    print_commands();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap(); // Ensures the user's input is on the same line as the prompt

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let line: Vec<&str> = buffer.trim().split_whitespace().collect();

        if let Some(command) = line.get(0) {
            match command.to_owned() {
                "new" => {
                    if let Ok(values) = parse_combo(line) {
                        print_info("Using provided combination (* * *)");
                        lock = Lock::new(values[0], values[1], values[2]);
                    } else {
                        print_info("Using default combination (0 0 0)");
                        lock = Lock::new(0,0,0);
                    }
                    lock.print_status();
                },
                "set" => {
                    let parsed = parse_combo(line);
                    if let Ok(values) = parsed {
                        lock.update_combination(values[0], values[1], values[2]);
                    } else if let Err(msg) = parsed {
                        print_error(&msg);
                    }
                    lock.print_status();
                }
                "open" => {
                    lock.open();
                    lock.print_status();
                },
                "close" => {
                    lock.close();
                    lock.print_status();
                },
                "lock" => {
                    lock.lock();
                    lock.print_status();
                },
                "unlock" => {
                    let parsed = parse_combo(line);
                    if let Ok(values) = parsed {
                        lock.unlock(values[0], values[1], values[2]);
                    } else if let Err(msg) = parsed {
                        print_error(&msg);
                    }
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
            Err("Error: Values must be numeric".to_owned())
        }
    } else {
        Err("Error: Three values required".to_owned())
    }
}

fn print_commands() {
    println!("Possible Commands: [new, set, open, close, lock, unlock, info, debug, quit, help]")
}

fn print_info(message: &str) {
    println!("[INFO] {}", message);
}
fn print_error(error: &str) {
    println!("[ERROR] {}", error);
}