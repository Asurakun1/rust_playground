fn filter_test(){
    let map: HashMap<i32, i32> = (0..10).map(|x| (x, x)).collect();
    let mut filter: Vec<(&i32, &i32)> = map.iter().filter(|(k, _v)| *k % 2 == 0).collect();
    filter.sort_by(|a, b| b.cmp(a));

    println!("{:?}", filter);
}