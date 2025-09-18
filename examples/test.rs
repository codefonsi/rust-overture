#[derive(Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

struct UserBuilder {
    id: u32,
    name: String,
    active: bool,
}

impl UserBuilder {
    fn new() -> Self {
        Self {
            id: 0,
            name: "Test User".to_string(),
            active: false,
        }
    }

    fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    fn build(self) -> User {
        User {
            id: self.id,
            name: self.name,
            active: self.active,
        }
    }
}

#[test]
fn test_user_builder() {
    let user = UserBuilder::new().id(1).name("Bob").active(true).build();
    assert_eq!(user.id, 1);
    assert!(user.active);
}

fn main() {}
