use crate::prelude::*;

#[derive(Debug)]
pub struct AuthService {
    pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        unimplemented!()
    }

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Empty, Status> {
        unimplemented!()
    }

    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Empty, Status> {
        unimplemented!()
    }
}
