use blocksmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME")
        .expect("ESUP_MULTI_USERNAME non défini dans l'environnement");
    let password = std::env::var("ESUP_MULTI_PASSWORD")
        .expect("ESUP_MULTI_PASSWORD non défini dans l'environnement");

    println!("1. Authentification initiale avec identifiant et mot de passe...");
    let initial_client = blocksmulti::Client::login(server_url.clone(), username, password, None).await?;
    let saved_token = initial_client.auth_token();
    println!("   Token obtenu avec succès: {}", saved_token);

    println!("\n=== 🔑 AUTHENTIFICATION VIA TOKEN (FROM_TOKEN) ===");
    println!("2. Création d'une nouvelle session Client avec le token sauvegardé...");
    let client = blocksmulti::Client::from_token(server_url, saved_token)?;
    println!("   Session Client reconstituée avec le token: {}", client.auth_token());

    println!("\n3. Validation des appels de l'API avec le client authentifié par token...");

    // Test 1: Actualités importantes
    match client.get_important_news().await {
        Ok(news) => println!("   ✅ Actualités importantes: {} élément(s) récupéré(s)", news.len()),
        Err(e) => eprintln!("   ❌ Échec récupération actualités: {:?}", e),
    }

    Ok(())
}
