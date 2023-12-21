#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: u32,
}

fn print_employee(employee: Employee){
    println!("{:?}", employee);
}

pub fn exec()
{
    let me = Employee {
        position: Position::Worker,
        work_hours: 8,
    };
    
    print_employee(me);
    print_employee(me);
}
