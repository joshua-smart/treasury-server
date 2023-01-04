use rocket::{
    delete,
    futures::lock::Mutex,
    get,
    http::Status,
    post, put,
    serde::{json::Json, Deserialize},
    State,
};
use treasury_service::{
    data_structures::{Id, Money, Transaction},
    TreasuryService,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostTransaction<'r> {
    amount: Money,
    datetime: &'r str,
}

#[post("/transactions", data = "<post_transaction>")]
pub async fn post_transaction(
    service: &State<Mutex<TreasuryService>>,
    post_transaction: Json<PostTransaction<'_>>,
) -> Result<(), (Status, String)> {
    service
        .lock()
        .await
        .add_transaction(post_transaction.amount, post_transaction.datetime)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))
}

#[get("/transactions")]
pub async fn get_transactions(
    service: &State<Mutex<TreasuryService>>,
) -> Result<Json<Vec<Transaction>>, (Status, String)> {
    service
        .lock()
        .await
        .get_transactions()
        .await
        .map(Json)
        .map_err(|e| (Status::InternalServerError, e.to_string()))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PutTransaction<'r> {
    id: Id,
    amount: Money,
    date: &'r str,
}

#[put("/transactions", data = "<put_transaction>")]
pub async fn put_transaction(
    service: &State<Mutex<TreasuryService>>,
    put_transaction: Json<PutTransaction<'_>>,
) -> Result<(), (Status, String)> {
    service
        .lock()
        .await
        .set_transaction(
            put_transaction.id,
            put_transaction.amount,
            put_transaction.date,
        )
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))
}

#[delete("/transactions", data = "<delete_transaction>")]
pub async fn delete_transaction(
    service: &State<Mutex<TreasuryService>>,
    delete_transaction: Json<Id>,
) -> Result<(), (Status, String)> {
    service
        .lock()
        .await
        .remove_transaction(*delete_transaction)
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))
}
