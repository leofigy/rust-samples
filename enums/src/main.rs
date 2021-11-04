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
}
