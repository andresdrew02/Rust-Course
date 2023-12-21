struct Ticket {
    event: String,
    price: f32,
}
enum Discount {
    Flat(i32),
}

pub fn exec() 
{
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount: {:?}", amount),
    };

    let concert = Ticket { event: "concert".to_owned(), price: 50.52 };
    match concert {
        Ticket { price: 50.52, event} => println!("event @ 50 = {:?}", event),
        Ticket { price, ..} => println!("price: {:?}", price),
    }
}