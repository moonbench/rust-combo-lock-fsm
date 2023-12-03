use std::fmt::Debug;
use std::io::{self};

#[derive(Debug)]
struct ComboLock {
    is_open: bool,
    is_locked: bool,
    state: Option<Box<dyn State>>,
    combination: [i8; 3],
}

impl ComboLock {
    pub fn new(first: i8, second: i8, third: i8) -> ComboLock {
        println!("--> Creating a new lock");
        ComboLock {
            is_open: true,
            is_locked: false,
            state: Some(Box::new(UnlockedOpen { })),
            combination: [first, second, third] 
        }
    }

    pub fn lock(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.lock());
        }
        self.update();
    }

    pub fn unlock(&mut self, first: i8, second: i8, third: i8) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.unlock(first, second, third, self.combination));
        }
        self.update();
    }

    pub fn close(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.close());
        }
        self.update();
    }

    pub fn open(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.open());
        }
        self.update();
    }

    pub fn update_combination(&mut self, first: i8, second: i8, third: i8) {
        if let Some(s) = self.state.take() {
            s.update_combination(self, first, second, third);
            self.state = Some(s);
        }
    }
    
    fn update(&mut self) {
        if let Some(s) = self.state.take() {
            self.is_open = s.is_open();
            self.is_locked = s.is_locked();
            self.state = Some(s);
        }
    }

    fn print_status(&self) {
        println!("--> Is open: {:?}    Is locked: {:?}", self.is_open, self.is_locked);
    }
}

trait State {
    fn name(&self) -> String;
    fn lock(self: Box<Self>) -> Box<dyn State>;
    fn unlock(self: Box<Self>, first: i8, second: i8, third: i8, combination: [i8; 3]) -> Box<dyn State>;
    fn close(self: Box<Self>) -> Box<dyn State>;
    fn open(self: Box<Self>) -> Box<dyn State>;
    fn update_combination(self: &Self, _lock: &mut ComboLock, _first: i8, _second: i8, _third: i8) {
        println!("--> Lock must be open to change the combination");
    }
    fn is_open(&self) -> bool { false }
    fn is_locked(&self) -> bool { false }
}

struct UnlockedOpen {}
impl State for UnlockedOpen {
    fn name(&self) -> String {
        "UnlockedOpen".to_owned()
    }
    fn lock(self: Box<Self>) -> Box<dyn State> {
        println!("--> Unable to lock an open lock");
        self
    }
    fn unlock(self: Box<Self>, _first: i8, _second: i8, _third: i8, _combination: [i8; 3]) -> Box<dyn State> {
        println!("--> Already unlocked");
        self
    }
    fn close(self: Box<Self>) -> Box<dyn State> {
        println!("--> Lock closed");
        Box::new(UnlockedClosed {})
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        println!("--> Lock already open");
        self
    }
    fn update_combination(self: &Self, lock: &mut ComboLock, first: i8, second: i8, third: i8) {
        println!("--> Combination changed");
        lock.combination = [first, second, third];
    }
    fn is_open(&self) -> bool {
        true
    }
}

struct UnlockedClosed {}
impl State for UnlockedClosed {
    fn name(&self) -> String {
        "UnlockedClosed".to_owned()
    }
    fn lock(self: Box<Self>) -> Box<dyn State> {
        println!("--> Locked!");
        Box::new(Locked {})
    }
    fn unlock(self: Box<Self>, _first: i8, _second: i8, _third: i8, _combination: [i8; 3]) -> Box<dyn State> {
        println!("--> Already unlocked");
        self
    }
    fn close(self: Box<Self>) -> Box<dyn State> {
        println!("--> Lock already closed");
        self
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        println!("--> Lock opened");
        Box::new(UnlockedOpen {})
    }
}

struct Locked {}
impl State for Locked {
    fn name(&self) -> String {
        "Locked".to_owned()
    }
    fn lock(self: Box<Self>) -> Box<dyn State> {
        println!("--> Lock already locked");
        self
    }
    fn unlock(self: Box<Self>, first: i8, second: i8, third: i8, combination: [i8; 3]) -> Box<dyn State> {
        if [first, second, third] == combination{
            println!("--> Valid combination, unlocked!");
            Box::new(UnlockedClosed {})
        } else {
            println!("--> Invalid combination, still locked");
            self
        }
    }
    fn close(self: Box<Self>) -> Box<dyn State> {
        println!("--> Lock already closed (and locked)");
        self
    }
    fn open(self: Box<Self>) -> Box<dyn State> {
        println!("--> Unable to open a locked lock");
        self
    }
    fn is_locked(&self) -> bool {
        true
    }
}


impl Debug for dyn State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn main() {
    let mut lock = ComboLock::new(0, 0, 0);

    println!("Finite State Machine Lock Demo");
    print_commands();

    loop {
        println!("Enter command: ");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let line: Vec<&str> = buffer.trim().split_whitespace().collect();

        if let Some(command) = line.get(0) {
            match command.to_owned() {
                "new" => {
                    if let Ok(values) = parse_combo(line) {
                        println!("Using provided combination (* * *)");
                        lock = ComboLock::new(values[0], values[1], values[2]);
                    } else {
                        println!("Using default combination (0 0 0)");
                        lock = ComboLock::new(0,0,0);
                    }
                },
                "set" => {
                    let parsed = parse_combo(line);
                    if let Ok(values) = parsed {
                        lock.update_combination(values[0], values[1], values[2]);
                    } else if let Err(msg) = parsed {
                        println!("{}", msg);
                    }
                }
                "open" => {
                    lock.open();
                },
                "close" => {
                    lock.close();
                },
                "lock" => {
                    lock.lock();
                },
                "unlock" => {
                    let parsed = parse_combo(line);
                    if let Ok(values) = parsed {
                        lock.unlock(values[0], values[1], values[2]);
                    } else if let Err(msg) = parsed {
                        println!("{}", msg);
                    }
                },
                "info" | "status" => lock.print_status(),
                "debug" => println!("{:#?}", lock),
                "exit" | "quit" => break,
                "h" | "help" | "?" => print_commands(),
                _ => println!("Unknown Command ({}) ('help' for help)", buffer.trim()),
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