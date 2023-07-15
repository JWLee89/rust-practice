fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct User {
    active: bool,
    username: String,
    password: String,
    sign_in_count: u64,
}

fn build_user(username: String, password: String) -> User {
    User {
        active: true,
        username,
        password,
        sign_in_count: 0
    }
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    let mut user = User {
        active: true,
        username: String::from("yee"),
        password: String::from("yee"),
        sign_in_count: 28,
    };
    let username = String::from("yeeeee");
    let password = String::from("cow");
    let mut new_user = build_user(username, password);

    let user_3 = User {
        password: String::from("pasword"),
        ..user
    };

    let rect = Rectangle {width: 10, height: 20};
    let area = calculate_area(&rect);
    println!("Area: {area}");
    println!("Rectangle: {:?}", rect);


   // println!("Username: {user.username}, password: {user.password}");

}
