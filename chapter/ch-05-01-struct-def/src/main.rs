fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("clock"),
        email: String::from("clock@abc.com"),
        sign_in_count: 23,
    };

    user1.email = String::from("another@def.com");

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@bbb.com"),
        ..user1
    };

    let color = Color(1, 1, 1);
    let point = Point(2., 3., 5.);

    let always_equal = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(f32, f32, f32);

struct AlwaysEqual;

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
