use crate::List::*;

enum Action{
    Drive,
    Turn(Direction),
    Stop
}

enum Direction{
    Left,
    Right
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64 },
}

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List{
    fn new() -> List {
        Nil
    }

    fn add(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32{

        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{} , {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("hello a new page"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".",s ),
        WebEvent::Click {x, y} => {
            println!("clicked x={} y={}", x, y);
        },
    }
}

fn print_action(a: Action){
    match a{
        Action::Drive => println!("Driving forward!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("going left 'pal ..."),
            Direction::Right => println!("going to the right 'pal"),
        },
        Action::Stop => println!("Stop 'pal"),
    }
}

fn main() {
    let flow = vec![Action::Drive, 
    Action::Turn(Direction::Left), 
    Action::Drive, 
    Action::Stop,
    Action::Turn(Direction::Right)
    ];

    for action in flow{
        print_action(action);
    }

    let pressed = WebEvent::KeyPress('X');
    let pasted = WebEvent::Paste("my cool text".to_owned());
    let click = WebEvent::Click {x : 20, y: 90};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let mut list = List::new();
    list = list.add(1);
    list = list.add(2);
    list = list.add(3);

    println!("Linked list : {}", list.len());
    println!("{}", list.stringify());

}
