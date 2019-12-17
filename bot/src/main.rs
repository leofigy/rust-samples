enum Action{
    Drive,
    Turn(Direction),
    Stop
}

enum Direction{
    Left,
    Right
}

fn print_action(a: Action){
    match a{
        Action::Drive => println!("Driving forward!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("going left sucker ..."),
            Direction::Right => println!("going to the right mother s.."),
        },
        Action::Stop => println!("Stop sucker!"),
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
}
