use rust_on_vercel_template::{
    auth::{extract_bearer_token, verify_jwt},
    create_pool, ApiError, UserRepository,
};
use serde_json;
use uuid::Uuid;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Only allow GET requests
    if req.method() != "GET" {
        let error = ApiError {
            message: "Method not allowed".to_string(),
            code: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
        };
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&error)?.into())?);
    }

    // Get authorization header
    let auth_header = match req.headers().get("authorization") {
        Some(header) => match header.to_str() {
            Ok(h) => h,
            Err(_) => {
                let error = ApiError {
                    message: "Invalid authorization header".to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                };
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&error)?.into())?);
            }
        },
        None => {
            let error = ApiError {
                message: "Authorization header required".to_string(),
                code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?);
        }
    };

    // Extract token from Bearer header
    let token = match extract_bearer_token(auth_header) {
        Ok(token) => token,
        Err(_) => {
            let error = ApiError {
                message: "Invalid authorization header format".to_string(),
                code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?);
        }
    };

    // Verify JWT token
    let claims = match verify_jwt(token) {
        Ok(claims) => claims,
        Err(_) => {
            let error = ApiError {
                message: "Invalid or expired token".to_string(),
                code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?);
        }
    };

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

    // Get user from database
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            let error = ApiError {
                message: "Invalid user ID in token".to_string(),
                code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?);
        }
    };

    match user_repo.get_user_by_id(&user_id).await {
        Ok(Some(user)) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&user)?.into())?),
        Ok(None) => {
            let error = ApiError {
                message: "User not found".to_string(),
                code: StatusCode::NOT_FOUND.as_u16(),
            };
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?)
        }
        Err(_) => {
            let error = ApiError {
                message: "Database error".to_string(),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?)
        }
    }
}
