use blocksmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME")
        .expect("ESUP_MULTI_USERNAME non défini dans l'environnement");
    let password = std::env::var("ESUP_MULTI_PASSWORD")
        .expect("ESUP_MULTI_PASSWORD non défini dans l'environnement");

    println!("Authentification initiale sur le serveur {}...", server_url);
    let client = blocksmulti::Client::login(server_url, username.clone(), password.clone(), None).await?;
    println!("Authentifié avec succès ! Token initial: {}", client.auth_token());

    println!("\n=== 🔄 TEST DE RECONNECTIVITÉ (RELOGIN) ===");
    println!("Re-connexion avec le serveur...");
    let new_client = client.relogin(username, password, None).await?;
    println!("Re-connecté avec succès ! Nouveau Token: {}", new_client.auth_token());

    Ok(())
}
