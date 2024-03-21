#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError>;
    async fn create(&self, new_user: CreateUserRequest) -> Result<String, CustomError>;
    async fn delete(&self, id: &str) -> Result<(), CustomError>;
    async fn authenticate(&self, credentials: AuthenticateUserRequest) -> Result<AuthenticateUserResponse, CustomError>;
}
impl UserServiceTrait for UserService {
    async fn create(&self, new_user: CreateUserRequest) -> Result<String, CustomError> {

    }
}