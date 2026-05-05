// File: src/api/server.rs
// Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Serialize)]
struct FormulaResponse {
    formula: String,
    r_squared: f64,
    inputs: Vec<String>,
}

#[derive(Serialize)]
struct PatternSummaryResponse {
    description: String,
    region: String,
    palettes: Vec<String>,
    formulas: serde_json::Value,
}

#[derive(Deserialize)]
struct GetFormulaQuery {
    pattern: String,
    parameter: String,
}

// GET /api/v1/formula?pattern=zombie-vomit&parameter=maskRadius
async fn get_formula(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetFormulaQuery>,
) -> Result<Json<FormulaResponse>, StatusCode> {
    let row = sqlx::query!(
        r#"
        SELECT fc.formula_simplified, fc.r_squared, fc.input_variables
        FROM formula_catalog fc
        JOIN pattern_catalog pc ON fc.pattern_id = pc.pattern_id
        WHERE pc.pattern_name = ? AND fc.parameter_name = ?
        "#,
        params.pattern,
        params.parameter
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match row {
        Some(r) => {
            let inputs: Vec<String> = serde_json::from_str(&r.input_variables)
                .unwrap_or_default();
            
            Ok(Json(FormulaResponse {
                formula: r.formula_simplified,
                r_squared: r.r_squared,
                inputs,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// GET /api/v1/pattern/{name}/summary
async fn get_pattern_summary(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<PatternSummaryResponse>, StatusCode> {
    let pattern = sqlx::query!(
        "SELECT description, default_region, palette_groups FROM pattern_catalog WHERE pattern_name = ?",
        name
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    let formulas = sqlx::query!(
        "SELECT parameter_name, formula_simplified FROM formula_catalog fc JOIN pattern_catalog pc ON fc.pattern_id = pc.pattern_id WHERE pc.pattern_name = ?",
        name
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let formula_map: serde_json::Value = formulas.iter()
        .map(|f| (f.parameter_name.clone(), f.formula_simplified.clone()))
        .collect::<Vec<_>>()
        .into();
    
    Ok(Json(PatternSummaryResponse {
        description: pattern.description,
        region: pattern.default_region,
        palettes: serde_json::from_str(&pattern.palette_groups).unwrap_or_default(),
        formulas: formula_map,
    }))
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "constellation_index.db".to_string());
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    let state = Arc::new(AppState { db: pool });
    
    let app = Router::new()
        .route("/api/v1/formula", get(get_formula))
        .route("/api/v1/pattern/:name/summary", get(get_pattern_summary))
        .route("/health", get(|| async { "OK" }))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    
    println!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
