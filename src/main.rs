use core::fmt;

struct Greeting {
    name: String,
}

impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Greeting {
            name: name.as_ref().to_string(),
        }
    }
}

impl fmt::Display for Greeting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello, {}!", self.name)
    }
}

fn main() {
    let greeting = Greeting::new("Kigali");
    println!("{}", greeting);
}
