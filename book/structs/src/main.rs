fn main() {
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };
  user1.email = String::from("anotheremail@example.com");
  let user2 = build_user(String::from("m@m.m"), String::from("m"));
  println!("{}, {}", user1.email, user2.username);
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
