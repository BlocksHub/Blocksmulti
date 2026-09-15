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

    println!("\n=== 🔴 ACTUALITÉS IMPORTANTES ===");
    match client.get_important_news().await {
        Ok(news) => {
            println!("{} actualité(s) importante(s) trouvée(s) :", news.len());
            for item in news {
                println!("- ID: {}", item.id);
                if let Some(translations) = item.translations {
                    for t in translations {
                        let lang = t.languages_code.as_deref().unwrap_or("fr");
                        println!("  [{}] {}", lang, t.title);
                        println!("  Content: {}", t.content);
                    }
                }
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des actualités importantes : {:?}", e),
    }

    println!("\n=== 📰 ACTUALITÉS CLASSIQUES (RSS) ===");
    match client.get_rss().await {
        Ok(rss_items) => {
            println!("{} article(s) RSS trouvé(s) :", rss_items.len());
            for item in rss_items {
                println!("- Titre: {}", item.title);
                if let Some(date) = &item.pub_date {
                    println!("  Date: {}", date);
                }
                if let Some(link) = &item.link {
                    println!("  Lien: {}", link);
                }
                if let Some(content) = &item.content {
                    println!("  Résumé: {}", content);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération du flux RSS : {:?}", e),
    }

    Ok(())
}
