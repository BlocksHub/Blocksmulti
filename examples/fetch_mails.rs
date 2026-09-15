use blockmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME")
        .expect("ESUP_MULTI_USERNAME non défini dans l'environnement");
    let password = std::env::var("ESUP_MULTI_PASSWORD")
        .expect("ESUP_MULTI_PASSWORD non défini dans l'environnement");

    println!("Authentification sur le serveur {}...", server_url);
    let client = blockmulti::Client::login(server_url, username, password, None).await?;
    println!("Authentifié avec succès !");

    println!("\n=== ✉️ MESSAGERIE & CALENDRIER ===");
    match client.get_mail_calendar().await {
        Ok(reply) => {
            if let Some(err) = reply.error {
                println!("Note du serveur: {}", err);
            }
            println!("📫 Mails non lus: {}", reply.unread_mails);
            println!("📅 Evénement(s) de calendrier: {}", reply.events.len());
            for event in reply.events {
                println!("- {}", event.label);
                if let Some(start) = event.start_date_time {
                    println!("  Début: {}", start);
                }
                if let Some(end) = event.end_date_time {
                    println!("  Fin: {}", end);
                }
                if let Some(loc) = event.location {
                    println!("  Lieu: {}", loc);
                }
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des mails/calendrier: {:?}", e),
    }

    Ok(())
}
