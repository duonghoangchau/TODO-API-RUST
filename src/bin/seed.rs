use std::{env, fs, path::Path};

use sqlx::PgPool;

const SEED_FILES: [&str; 3] = [
    "seeds/seed_roles.sql",
    "seeds/seed_todo_categories.sql",
    "seeds/seed_todo_priorities.sql",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    for seed_file in SEED_FILES {
        let path = Path::new(seed_file);
        let script = fs::read_to_string(path)?;
        sqlx::query(&script).execute(&pool).await?;
        println!("Applied {}", path.display());
    }

    println!("Seed completed successfully.");

    Ok(())
}
