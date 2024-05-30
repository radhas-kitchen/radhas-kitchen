use crate::proto::*;
use log::*;
use thiserror::Error;

const URL: &str = "http://localhost:50051";

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Failed to connect to server: {0}")]
    ConnectError(#[from] tonic::transport::Error),

    #[error("Failed to send request: {0}")]
    RequestError(#[from] tonic::Status),
}

impl CommandError {
    pub fn message(&self) -> String {
        match self {
            CommandError::ConnectError(_) => "Failed to connect to server".to_string(),
            CommandError::RequestError(e) => e.to_string(),
        }
    }
}

macro_rules! command_handler2 {
    ($serv:ident :: $rpc:ident, $fnn:ident () -> Vec<$ret:ty>) => {
        #[tauri::command]
        pub async fn $fnn() -> Result<Vec<$ret>, String> {
            async fn inner() -> Result<Vec<$ret>, CommandError> {
                let mut client = $serv::connect(URL).await?;
                let res = client.$rpc(Empty::default()).await?;

                let mut res = res.into_inner();
                let mut vec = vec![];

                while let Some(item) = res.message().await? {
                    vec.push(item);
                }

                Ok(vec)
            }

            inner().await.map_err(|e| e.message().to_string())
        }
    };

    ($serv:ident :: $rpc:ident, $fnn:ident ($in:ty)) => {
        #[tauri::command]
        pub async fn $fnn(request: $in) -> Result<(), String> {
            async fn inner(req: $in) -> Result<(), CommandError> {
                let mut client = $serv::connect(URL).await?;
                client.$rpc(req).await?;

                Ok(())
            }

            inner(request).await.map_err(|e| e.message().to_string())
        }
    };

    ($serv:ident :: $rpc:ident, $fnn:ident () -> $ret:ty) => {
        #[tauri::command]
        pub async fn $fnn() -> Result<$ret, String> {
            async fn inner() -> Result<$ret, CommandError> {
                let mut client = $serv::connect(URL).await?;
                let res = client.$rpc(Empty::default()).await?;

                Ok(res)
            }

            inner().await.map_err(|e| e.message().to_string())
        }
    };

    ($serv:ident :: $rpc:ident, $fnn:ident ($in:ty) -> $ret:ty) => {
        #[tauri::command]
        pub async fn $fnn(request: $in) -> Result<$ret, String> {
            async fn inner(req: $in) -> Result<$ret, CommandError> {
                let mut client = $serv::connect(URL).await?;
                let response = client.$rpc(req).await?;
                Ok(response.into_inner())
            }

            inner(request).await.map_err(|e| e.message().to_string())
        }
    };
}

command_handler2!(AuthClient::login, auth_login(LoginRequest) -> LoginResponse);
command_handler2!(AuthClient::create_user, auth_create_user(CreateUserRequest));

command_handler2!(JobsClient::jobs, jobs_list() -> Vec<Job>);
command_handler2!(JobsClient::get, jobs_get(JobId) -> Job);
command_handler2!(JobsClient::post, jobs_post(Authorization));
command_handler2!(JobsClient::cancel, jobs_cancel(JobUpdateRequest));
command_handler2!(JobsClient::claim, jobs_claim(JobUpdateRequest));
command_handler2!(JobsClient::unclaim, jobs_unclaim(JobUpdateRequest));
command_handler2!(JobsClient::pickup, jobs_pickup(JobUpdateRequest));
command_handler2!(JobsClient::dropoff, jobs_dropoff(JobUpdateRequest));
