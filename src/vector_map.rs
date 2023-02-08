
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn ax() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let v1_iter = &v1.iter();

    let v2: Vec<_> = v1.iter().map(|x| x % 2).collect();
    println!("{:?}", v2);
}
