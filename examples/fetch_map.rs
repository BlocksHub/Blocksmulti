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

    println!("\n=== 🗺️ CARTE / MAP ===");
    match client.get_map().await {
        Ok(map_data) => {
            if !map_data.items.is_empty() {
                println!("{} point(s) d'intérêt (items) trouvé(s) :", map_data.items.len());
                for (i, item) in map_data.items.iter().enumerate().take(10) {
                    let cat = item.category.as_deref().unwrap_or("Non catégorisé");
                    let title = item
                        .title
                        .as_ref()
                        .and_then(|t| t.iter().find(|tr| tr.langcode.as_deref() == Some("fr")).or_else(|| t.first()))
                        .and_then(|tr| tr.value.as_deref())
                        .unwrap_or("Sans titre");

                    println!(
                        "  [{}] ({}) {} -> lat: {}, lng: {}",
                        i + 1,
                        cat,
                        title,
                        item.latitude,
                        item.longitude
                    );
                }
                if map_data.items.len() > 10 {
                    println!("  ... et {} autre(s) point(s)", map_data.items.len() - 10);
                }
            }

            if !map_data.campuses.is_empty() {
                println!("\nCampuses: {}", map_data.campuses.len());
                for campus in &map_data.campuses {
                    println!("  - [{}] {}", campus.id, campus.name);
                }
            }

            if !map_data.markers_collections.is_empty() {
                println!("\nMarkers Collections: {} collection(s)", map_data.markers_collections.len());
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération de la carte : {:?}", e),
    }

    Ok(())
}
