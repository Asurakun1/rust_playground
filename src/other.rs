pub fn other() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    let result = add(10, 3);
    println!("{}", result);

    let multiplier = 2;
    let double = |x: i32| x * multiplier;
    let result = double(3);

    println!("{}", result);

    let mut numbers = vec![1, 5, 3, 2, 5, 6];
    numbers.sort_by(|a, b| a.cmp(b));
    let z = numbers;

    println!("{:?}", z);

    let multiplier = 2;
    let double = move |x: i32| x * &multiplier;
    let result = double(3);

    let data = vec![1, 2, 3];
    let closure = move || println!("Captured {data:?} by value");

    println!("Result: {:?}", result);

    closure();
}
