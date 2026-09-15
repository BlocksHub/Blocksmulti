use blockmulti::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let server_url = std::env::var("ESUP_MULTI_SERVER_URL")
        .unwrap_or_else(|_| "https://appmob.uphf.fr/backend/".to_string());
    let username = std::env::var("ESUP_MULTI_USERNAME").ok();
    let password = std::env::var("ESUP_MULTI_PASSWORD").ok();

    let client = match (username, password) {
        (Some(u), Some(p)) if !u.is_empty() && !p.is_empty() => {
            println!("Authentification sur le serveur {}...", server_url);
            let c = blockmulti::Client::login(server_url, u, p, None).await?;
            println!("Authentifié avec succès !");
            c
        }
        _ => {
            println!("Initialisation du client non authentifié sur {}...", server_url);
            blockmulti::Client::new_unauthenticated(server_url)?
        }
    };

    println!("\n=== ✉️ CONTACT / CONTACT US ===");

    // 1. Récupération des informations de la page de contact
    match client.get_contact_us_page().await {
        Ok(page) => {
            if let Some(icon) = &page.icon {
                println!("Icône: {}", icon);
            }
            println!("Traductions de la page de contact ({}):", page.translations.len());
            for t in &page.translations {
                let lang = t.languages_code.as_deref().unwrap_or("fr");
                println!("\n  [{}] {}", lang, t.title);
                println!("  Contenu HTML: {}", t.content.trim());
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération de la page de contact : {:?}", e),
    }

    Ok(())
}
