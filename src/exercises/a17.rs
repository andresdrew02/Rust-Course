fn transform_to_lowercase(s: &str) -> String 
{
    s.to_lowercase()
}

fn transform_to_uppercase(s: &str) -> String 
{
    s.to_uppercase()
}

pub fn exec()
{
    let s = "hello, world!";
    println!("{}", transform_to_lowercase(s));
    println!("{}", transform_to_uppercase(s));
}