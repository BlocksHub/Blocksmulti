use blocksmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME").expect("username not defined");
    let password = std::env::var("ESUP_MULTI_PASSWORD").expect("password not defined");
    let client = blocksmulti::Client::login(server_url, username, password, None).await?;

    println!("Logged in! Token: {}", client.auth_token());
    Ok(())
}
