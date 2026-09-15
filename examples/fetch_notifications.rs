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

    println!("\n=== 🔔 NOTIFICATIONS ===");
    match client.get_notifications(0, 10).await {
        Ok(notifs) => {
            println!("{} notification(s) trouvée(s) :", notifs.len());
            for notif in notifs {
                println!("\n- ID: {}", notif.id);
                println!("  Canal: {}", notif.channel);
                println!("  Titre: {}", notif.title);
                println!("  Message: {}", notif.message.replace('\n', " "));
                if let Some(state) = &notif.state {
                    println!("  Statut: {}", state);
                }
                if let Some(date) = &notif.creation_date {
                    println!("  Date: {}", date);
                }
                if let Some(url) = &notif.url {
                    println!("  Lien: {}", url);
                }
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des notifications : {:?}", e),
    }

    Ok(())
}
