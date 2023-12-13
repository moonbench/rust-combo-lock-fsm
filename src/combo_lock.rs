use std::fmt::Debug;


#[derive(Debug)]
pub struct Lock {
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

    pub fn handle_event(&mut self, event: Event) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.handle_event(event, self));
        }
    }

    pub fn print_status(&self) {
        if let Some(s) = self.state.as_ref() {
            println!("{}", &s.draw());
        } else {
            unreachable!("The lock should never be in an invalid state");
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
        String::from(r#"         ________
        |  ____  |
        | |    | |
        |_|    | |
         ______|_| OPEN
        |   __   |
        |  [  ]  |
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
        String::from(r#"         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [  ]  |
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
        String::from(r#"         ________
        |  ____  |
        |_|____|_| CLOSED
        |   __   |
        |  [**]  |
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


pub enum Event {
    Lock,
    Unlock(i8, i8, i8),
    Open,
    Close,
    ChangeCode(i8, i8, i8),
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