
fn yee() {
    let fruits = vec!["banana", "apple", "orange", "coconut", "strawberry"];
    let yee = fruits.get(100);

    if yee == None {
        println!("Yee is none");
    }

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            // name of the fruit
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("Invalid index: {:?}", index),
        }
    }

    let num = Some(5);
    if let Some(7) = num {
        println!("Yee is 7");
    }

    assert_eq!(None.unwrap_or("default"), "default");
    // This will panic
    let gift: Option<&str> = None.expect("expected a gift. Got nothing");
}