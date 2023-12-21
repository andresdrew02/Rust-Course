enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64,String)
}

pub fn exec() 
{
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(100.0, "Backstage Ticket".to_owned()),
        Ticket::Standard(100.0),
        Ticket::Vip(100.0, "Vip Ticket".to_owned()),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price,holder) => println!("{} {}", holder, price),
            Ticket::Standard(price) => println!("{}", price),
            Ticket::Vip(price,holder) => println!("{} {}", holder, price),
        }
    }
}