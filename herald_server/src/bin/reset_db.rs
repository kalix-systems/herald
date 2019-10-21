use herald_server::store::get_client;

#[tokio::main]
async fn main() {
    let mut client = get_client()
        .await
        .expect("Failed to get client while trying to reset database");
    client.reset_all().await.expect("Failed to reset database");
}
