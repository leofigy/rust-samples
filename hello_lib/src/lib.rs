#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn foo(){
    println!("hello foo from library");
}

fn bar(){
    println!("this is a private bar");
}