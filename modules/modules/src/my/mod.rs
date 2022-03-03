pub mod nested;

fn private_foo(){
    println!("called in private inside a module");
}

pub fn foo(){
    println!("public foo inside module");
    private_foo();
}