use crate::proto::*;
use log::*;
use std::error::Error;
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
            CommandError::ConnectError(e) => "Failed to connect to server".to_string(),
            CommandError::RequestError(e) => e.to_string(),
        }
    }
}

macro_rules! command_handler {
    ($fnname:ident ($($arg:ident : $aty:ty),*) -> $ret:ty $inner_fn:block) => {
        #[tauri::command]
        pub async fn $fnname($($arg: $aty),*) -> Result<$ret, String> {
            async fn inner($($arg: $aty),*) -> Result<$ret, Box<dyn Error>> $inner_fn

            inner($($arg),*).await.map_err(|e| {
                error!("{}: {:?}", stringify!($fnname), e);
                format!("{}", e.message())
            })
        }
    };
}

macro_rules! command_handler2 {
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

command_handler2!(AuthClient::login, grpc_login(LoginRequest) -> LoginResponse);
command_handler2!(AuthClient::create_user, grpc_create_user(CreateUserRequest));

// command_handler!(grpc_login(request: LoginRequest) -> LoginResponse {
//     let mut client = AuthClient::connect(URL).await?;
//     let response = client.login(request).await?;

//     Ok(response.into_inner())
// });

// command_handler!(grpc_create_user(request: CreateUserRequest) -> () {
//     AuthClient::connect(URL).await?.create_user(request).await?;
//     Ok(())
// });

// #[tauri::command]
// pub async fn grpc_login(request: LoginRequest) -> Result<LoginResponse, String> {
//     let mut client = AuthClient::connect(URL)
//         .await
//         .map_err(|e| format!("{e:?}"))?;

//     let response = client.login(request).await.map_err(|e| e.to_string())?;

//     Ok(response.into_inner().into())
// }

// #[tauri::command]
// pub async fn grpc_create_user(request: CreateUserRequest) -> Result<(), String> {
//     debug!("grpc_create_user: got request: {:?}", request);

//     let mut client = AuthClient::connect(URL)
//         .await
//         .map_err(|e| format!("{e:?}"))?;

//     debug!("grpc_create_user: connected to server");

//     client
//         .create_user(request)
//         .await
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }
