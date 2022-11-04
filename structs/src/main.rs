struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Using struct update syntax to set a new email value for a User 
    // instance but use the rest of the values from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // change the value of the email field of the mutable user1 instance
    user1.email = String::from("anotheremail@example.com");
}

// A function that takes an email and username and returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Because the parameter names and the struct field names are exactly the same,
// we can use the field init shorthand syntax so that it behaves exactly the 
// same but doesn’t have the repetition of email and username

// same as fn build_user()
fn build_user_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    let user1 = User {
        email: String::from("another@example.com"),
        username: String::from("dotdc"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    user2
}
