use blocksmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME")
        .expect("ESUP_MULTI_USERNAME non défini dans l'environnement");
    let password = std::env::var("ESUP_MULTI_PASSWORD")
        .expect("ESUP_MULTI_PASSWORD non défini dans l'environnement");

    println!("Authentification sur le serveur {}...", server_url);
    let client = blocksmulti::Client::login(server_url, username, password, None).await?;
    println!("Authentifié avec succès !");

    println!("\n=== 🔑 TICKETS CAS (SSO SERVICE TICKETS) ===");

    let services = [
        ("Moodle", "https://moodle.uphf.fr"),
        ("ENT", "https://ent.uphf.fr"),
        ("Webmail", "https://webmail.uphf.fr/login"),
    ];

    for (name, service_url) in services {
        println!("\nService: {} ({})", name, service_url);
        match client.get_sso_service_token(service_url.to_string()).await {
            Ok(ticket) => println!("  🎫 Ticket CAS: {}", ticket),
            Err(e) => eprintln!("  ❌ Erreur lors de la génération du ticket: {:?}", e),
        }
    }

    Ok(())
}
