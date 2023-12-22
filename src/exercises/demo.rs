/// A favorite color
enum Color {
    Red,
    Blue,
}

/// A piece of mail
struct Mail {
    /// The address of the mail
    address: String,
    /// The color of the mail
    color: Color,
    /// The sum of the mail
    sum: i32,
}

/// Prints the mail
impl Mail {
    /// Prints the color
    fn print_color(&self) {
        match self.color {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
        }
    }
    /// Prints the mail information
    fn print(&self) {
        self.print_color();
        println!("{} {}", self.address, self.sum);
    }
}

/// Adds two numbers
fn add(a: i32, b:i32) -> i32 {
    return a+b
}

pub fn exec()
{
    let mail = Mail {
        address: "123 Main Street".to_string(),
        color: Color::Red,
        sum: add(1,2)
    };
    mail.print();
}