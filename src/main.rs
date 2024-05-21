use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use uuid::Uuid;
use crate::model::{Product, ProductData};
use crate::product_repo::{InMemoryProductRepo, ProductRepo};

mod model;
mod product_repo;

#[derive(Clone)]
struct AppState<T> {
    product_repo: T
}

async fn create_product<T>(
    State(state): State<AppState<T>>,
    Json(data) : Json<ProductData>
) -> Json<Product>
 where T: ProductRepo {
    let product = Product {
        id : Uuid::new_v4(),
        name: data.name
    };

    state.product_repo.save_product(&product);

    Json(product)
}

async fn get_product<T> (
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>
) -> Result<Json<Product>,StatusCode> where T: ProductRepo {
    match state.product_repo.get_product(id) {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    // Dependency Injection in RUST
    let product_repo = InMemoryProductRepo::default();

    let app : Router = Router::new()
        .route("/product/:id", get(get_product::<InMemoryProductRepo>))
        .route("/product", post(create_product::<InMemoryProductRepo>))
        .with_state(AppState {product_repo});

    let listener =
        TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening....{:?}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}
