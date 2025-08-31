use anyhow::Result;
use rust_on_vercel_template::{create_pool, run_migrations};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    println!("Connecting to database...");
    let pool = create_pool().await?;

    println!("Running migrations...");
    run_migrations(&pool).await?;

    println!("Migrations completed successfully!");
    Ok(())
}
