use blocksmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME").ok();
    let password = std::env::var("ESUP_MULTI_PASSWORD").ok();

    let client = match (username, password) {
        (Some(u), Some(p)) if !u.is_empty() && !p.is_empty() => {
            println!("Authentification sur le serveur {}...", server_url);
            let c = blocksmulti::Client::login(server_url, u, p, None).await?;
            println!("Authentifié avec succès !");
            c
        }
        _ => {
            println!("Initialisation du client non authentifié sur {}...", server_url);
            blocksmulti::Client::new_unauthenticated(server_url)?
        }
    };

    println!("\n=== 📊 ENVOI DE STATISTIQUES D'USAGE ===");
    let duid = "device_test_123".to_string();
    let action = "OPEN".to_string();
    let functionality = "map".to_string();
    let platform = "ios".to_string();
    let connection_type = "wifi".to_string();

    println!("Envoi de la statistique d'ouverture du module 'map'...");
    match client
        .post_user_action_statistic(duid, action, functionality, platform, connection_type)
        .await
    {
        Ok(()) => println!("Statistique enregistrée avec succès par le serveur !"),
        Err(e) => eprintln!("Erreur lors de l'envoi de la statistique : {:?}", e),
    }

    Ok(())
}
