mod stack;

fn main() {
    let mut stack = stack::Stack::new();


    (0..10).map(|x| x.to_string())
        .for_each(|x| stack.push(x));
    println!("Stack size is {}", stack.size());
    stack.pop().unwrap();
    println!("Stack size is {}", stack.size());
    stack.display_all_contents();
    stack.push(String::from("5"));
    assert_eq!("5", *stack.top().unwrap())
}
