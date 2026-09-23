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

    println!("\n=== ⚙️ FONCTIONNALITÉS & WIDGETS (FEATURES) ===");
    match client.get_features().await {
        Ok(res) => {
            println!("\n📌 Features ({}) :", res.features.len());
            for feat in &res.features {
                let title = feat
                    .translations
                    .first()
                    .and_then(|t| t.title.as_deref())
                    .unwrap_or(&feat.id);
                let menu = feat.menu.as_deref().unwrap_or("aucun");
                println!("  - [{}] {} (menu: {}, pos: {})", feat.id, title, menu, feat.position);
            }

            println!("\n🧩 Widgets ({}) :", res.widgets.len());
            for widg in &res.widgets {
                let title = widg
                    .translations
                    .first()
                    .and_then(|t| t.title.as_deref())
                    .unwrap_or(&widg.id);
                let wtype = widg.widget.as_deref().unwrap_or("inconnu");
                println!("  - [{}] {} (type: {}, pos: {})", widg.id, title, wtype, widg.position);
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des fonctionnalités : {:?}", e),
    }

    Ok(())
}
