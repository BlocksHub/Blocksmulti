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

    println!("\n=== 🍴 RESTAURANTS UNIVERSITAIRES (CANTINE) ===");
    match client.get_restaurants().await {
        Ok(restaurants) => {
            println!("{} restaurant(s) trouvé(s) :", restaurants.len());
            for resto in &restaurants {
                println!("\n📍 [{}] {}", resto.id, resto.title);
                if let Some(zone) = &resto.zone {
                    println!("   Zone: {}", zone);
                }
                if let Some(desc) = &resto.short_desc {
                    println!("   Description: {}", desc);
                }
                if let Some(opening) = &resto.opening {
                    for (day, op) in opening {
                        println!("   Horaires {}: {} (ouvert: {})", day, op.label, op.is_open);
                    }
                }

                // Récupération des menus pour ce restaurant pour la date du jour
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                println!("   🍽️  Récupération des menus pour le {}...", today);
                match client.get_restaurant_menus(resto.id.to_string(), today).await {
                    Ok(menus) => {
                        for menu in menus {
                            println!("   - Menu du Date: {}", menu.date);
                            for meal in menu.meal {
                                println!("     Repas: {}", meal.name);
                                for cat in meal.foodcategory {
                                    println!("       Catégorie {}: {}", cat.name, cat.dishes.join(", "));
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("   (Impossible de charger les menus : {:?})", e),
                }
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des restaurants : {:?}", e),
    }

    Ok(())
}
