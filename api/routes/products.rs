use rust_on_vercel_template::{
    auth::{extract_bearer_token, verify_jwt},
    ApiError, Product,
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
    match verify_jwt(token) {
        Ok(_) => {
            // Token is valid, return sample products
            let products = vec![
                Product {
                    id: "1".to_string(),
                    name: "Laptop".to_string(),
                    price: 999.99,
                },
                Product {
                    id: "2".to_string(),
                    name: "Mouse".to_string(),
                    price: 29.99,
                },
                Product {
                    id: "3".to_string(),
                    name: "Keyboard".to_string(),
                    price: 79.99,
                },
            ];

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&products)?.into())?)
        }
        Err(_) => {
            let error = ApiError {
                message: "Invalid or expired token".to_string(),
                code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error)?.into())?)
        }
    }
}
