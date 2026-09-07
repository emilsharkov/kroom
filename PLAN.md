# Syringe

Syringe is a rust library utilizing IoC containers to complete dependency injection.



## Example

```rust
// Model
struct User {
    id: String;
    name: String;
}

// Repository interface
pub trait UserRepository {
    fn find_one(&self, id: &str) -> User;
}
#[injectable]
struct UserPgRepository;
impl UserRepository for UserPgRepository {
    fn find_one(&self, id: &str) -> User {
        return User {
            id: "123-456",
            name: "Emil"
        };
    }
}

// Service interface
pub trait UserService {
    fn find_one(&self, id: &str) -> User;
}
#[injectable]
struct DefaultUserService {
    #[inject]
    user_repo: UserRepository;
}
impl UserService for DefaultUserService {
    fn find_one(&self, id: &str) -> User {
        return self.user_repo.find_one(id);
    }
}

// Controller
enum GetUserByIdQuery {

}

enum GetUserByIdPath {

}


#[controller]
struct UserController {
    #[inject]
    user_service: UserService;
}

impl UserController {
    #[get("/{id}")
    async fn get_user(
        &self,
        Path(id): Path<String>,
        Query(query): Query<String>,
        Headers(headers): HeaderMap,
    ) -> Result<UserResponse,UserError> {

    }
}
```



