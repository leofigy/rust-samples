mod my;

fn foo(){
    println!("called from main ");
}

fn main() {
    foo();
    my::foo();
    my::nested::foo();
}

