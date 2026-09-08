# Syringe
Using IoC containers for dependency injection in Rust



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
struct UserPgRepository;
#[injectable]
impl UserRepository for UserPgRepository {
    fn find_one(&self, id: &str) -> User {
        User {
            id: "123-456".to_string(),
            name: "Emil".to_string(),
        }
    }
}

// Service interface
pub trait UserService {
    fn find_one(&self, id: &str) -> User;
}
struct DefaultUserService {
    #[inject]
    user_repository: Arc<dyn UserRepository>;
}
#[injectable]
impl UserService for DefaultUserService {
    fn find_one(&self, id: &str) -> User {
        self.user_repository.find_one(id)
    }
}

// Under the hood...
// Injectable creates a constructor for the implementation
// It uses the inject macro to figure out if/which things it needs from the container
// Injectable also registers the implementation to the interface to the constructor (somehow at compile time using inventory!)
trait Injectable {
    fn __syringe_construct(container: Container);
}
impl Injectable for DefaultUserService {
    fn __syringe_construct(container: Container) -> Self {
        Self {
            user_repository: container.get("UserRepository")
        }
    }
}

// Controller
struct UserController {
    #[inject]
    user_service: Arc<dyn UserService>;
}

#[controller("users")]
impl HttpController for UserController {
    #[get("/{id}")
    async fn get_user(
        &self,
        Path(GetUserByIdPath{ id }): Path<GetUserByIdPath>,
    ) -> Result<UserResponse,UserError> {
        self.user_service.find_one(id)
    }
}

// Under the hood...
trait HttpController: Injectable {
    fn registerRoutes();
}

impl HttpController for UserController {
    fn __syringe_construct(container: Container) -> Self {
        Self {
            user_service: container.get("UserService")
        }
    }

    async fn __get_user(
        Path(GetUserByIdPath{ id }): Path<GetUserByIdPath>,
        State(container): State<Container>,
    ) -> Result<UserResponse,UserError> {
        let user_controller = container.get("UserController")
        user_controller.get_user(
            Path(GetUserByIdPath{ id })
        )
    }
    async fn get_user(
        &self,
        Path(GetUserByIdPath{ id }): Path<GetUserByIdPath>,
    ) -> Result<UserResponse,UserError> {
        self.user_service.find_one(id)
    }
}
```
