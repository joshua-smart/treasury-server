mod paths;

use rocket::{futures::lock::Mutex, get, launch, routes};
use treasury_service::TreasuryService;

use paths::transactions::{
    delete_transaction, get_transactions, post_transaction, put_transaction,
};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let database_path =
        std::env::var("DATABASE_PATH").expect("DATABASE_PATH environment variable must be set");

    let service = TreasuryService::new(&database_path)
        .await
        .expect("Failed to start treasury-service");

    rocket::build().manage(Mutex::new(service)).mount(
        "/",
        routes![
            index,
            post_transaction,
            get_transactions,
            put_transaction,
            delete_transaction
        ],
    )
}
