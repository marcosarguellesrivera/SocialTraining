use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, env};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use uuid::Uuid;
use dotenvy::dotenv;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(FromRow)]
struct User {
    uuid: uuid:Uuid,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, String> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let user = user.ok_or_else(|| "Invalid credentials".to_string())?;

    // Check password
    let parsed_hash = PasswordHash::new(&user.password).map_err(|e| e.to_string())?;
    let argon2 = Argon2::default();

    if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return Err("Invalid credentials".to_string());
    }

    // Generate JWT
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.uuid.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET undefined")?;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| e.to_string())?;

    Ok(Json(LoginResponse { token }))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not setted");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the DB");

    let app = Router::new().route("/login", post(login)).with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));

    println!("Authservice listening at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await().unwrap(), app).await().unwrap();
}
