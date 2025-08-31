use rust_on_vercel_template::{
    auth::create_jwt, create_pool, ApiError, AuthResponse, CreateUserRequest, UserRepository,
};
use serde_json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Only allow POST requests
    if req.method() != "POST" {
        let error = ApiError {
            message: "Method not allowed".to_string(),
            code: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
        };
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&error)?.into())?);
    }

    // Parse request body
    let body = req.body();
    let create_user_request: CreateUserRequest = match serde_json::from_slice(body) {
        Ok(req) => req,
        Err(_) => {
            let error = ApiError {
                message: "Invalid request body".to_string(),
                code: StatusCode::BAD_REQUEST.as_u16(),
            };
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?);
        }
    };

    // Validate input
    if create_user_request.email.is_empty()
        || create_user_request.username.is_empty()
        || create_user_request.password.len() < 6
    {
        let error = ApiError {
            message: "Email, username are required and password must be at least 6 characters"
                .to_string(),
            code: StatusCode::BAD_REQUEST.as_u16(),
        };
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&error)?.into())?);
    }

    // Connect to database
    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            let error = ApiError {
                message: "Database connection failed".to_string(),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?);
        }
    };

    let user_repo = UserRepository::new(pool);

    // Create user
    match user_repo.create_user(create_user_request).await {
        Ok(user) => {
            // Generate JWT token
            let token = match create_jwt(&user.id, &user.email) {
                Ok(token) => token,
                Err(_) => {
                    let error = ApiError {
                        message: "Failed to generate token".to_string(),
                        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    };
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&error)?.into())?);
                }
            };

            let response = AuthResponse { user, token };

            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&response)?.into())?)
        }
        Err(e) => {
            let error = ApiError {
                message: e.to_string(),
                code: StatusCode::BAD_REQUEST.as_u16(),
            };
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?)
        }
    }
}
