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

    println!("\n=== 📄 PAGES STATIQUES (STATIC PAGES) ===");
    match client.get_static_pages().await {
        Ok(pages) => {
            println!("{} page(s) statique(s) trouvée(s) :", pages.len());
            for page in pages {
                let status = page.status.as_deref().unwrap_or("inconnu");
                let pos = page.position.map(|p| p.to_string()).unwrap_or_else(|| "aucun".to_string());
                println!("\n- ID: {}, Statut: {}, Position: {}", page.id, status, pos);
                if let Some(translations) = &page.translations {
                    for t in translations {
                        let lang = t.languages_code.as_deref().unwrap_or("fr");
                        println!("  [{}] {}", lang, t.title);
                        let content_snippet = if t.content.len() > 100 {
                            format!("{}...", &t.content[..100].replace('\n', " "))
                        } else {
                            t.content.replace('\n', " ")
                        };
                        println!("      Aperçu: {}", content_snippet);
                    }
                }
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des pages statiques : {:?}", e),
    }

    Ok(())
}
