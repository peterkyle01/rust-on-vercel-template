use rust_on_vercel_template::{
    auth::create_jwt, create_pool, ApiError, AuthResponse, LoginRequest, UserRepository,
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
    let login_request: LoginRequest = match serde_json::from_slice(body) {
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
    if login_request.email.is_empty() || login_request.password.is_empty() {
        let error = ApiError {
            message: "Email and password are required".to_string(),
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

    // Authenticate user
    match user_repo
        .authenticate_user(&login_request.email, &login_request.password)
        .await
    {
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
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&response)?.into())?)
        }
        Err(_) => {
            let error = ApiError {
                message: "Invalid credentials".to_string(),
                code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?)
        }
    }
}
