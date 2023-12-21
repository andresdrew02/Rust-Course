use std::vec;

struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

struct Student {
    name: String,
    locker: Option<i32>
}

fn first_example()
{
    let customers: Vec<Customer> = vec![
        Customer {
            age: Some(22),
            email: "pD6o0@example.com".to_owned(),
        },
        Customer {
            age: None,
            email: "pD6o0@example.com".to_owned(),
        }
    ];

    for customer in customers {
        match customer.age {
            Some(age) => println!("Customer age is {:?} and his email is {:?}", age , customer.email),
            None => println!("Customer has no age"),
        }
    }
}

fn find_quantity(name: &str) -> Option<i32>
{
    let grocies = vec![
        GroceryItem { name: "apple".to_owned(), qty: 10 },
        GroceryItem { name: "banana".to_owned(), qty: 5 },
        GroceryItem { name: "orange".to_owned(), qty: 7 },
    ];
    for item in grocies {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}

fn second_example()
{
    let qty = find_quantity("banana");
    if qty == None {
        println!("Item not found");
    }
    else {
        println!("{:?}", qty.unwrap());
    }
}

fn third_example()
{
    let response = Survey {
        q1: Some(1),
        q2: Some(true),
        q3: None
    };
    match response.q1 {
        Some(q1) => println!("q1: {:?}", q1),
        None => println!("q1: None"),
    }
    match response.q2 {
        Some(q2) => println!("q2: {:?}", q2),
        None => println!("q2: None"),
    }
    match response.q3 {
        Some(q3) => println!("q3: {:?}", q3),
        None => println!("q3: None"),
    }
}

fn get_lockers()
{
    let students: Vec<Student> = vec![
        Student {
            name: "Bule".to_owned(),
            locker: Some(1)
        },
        Student {
            name: "Julian".to_owned(),
            locker: None
        }
    ];
    for student in students {
        match student.locker {
            Some(locker) => println!("Student {:?} has locker: {:?}", student.name, locker),
            None => println!("Student {:?} has no locker", student.name),
        }
    }
}

pub fn exec()
{
    first_example();
    second_example();
    third_example();
    get_lockers();
}