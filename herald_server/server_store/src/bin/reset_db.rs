use server_store::*;

#[tokio::main]
async fn main() {
    let pool = Pool::new();
    let mut client = pool
        .get()
        .await
        .expect("Failed to get client while trying to reset database");
    client.reset_all().await.expect("Failed to reset database");
}
