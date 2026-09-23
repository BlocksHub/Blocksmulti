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

    println!("\n=== 🌐 RÉSEAUX SOCIAUX ===");
    match client.get_social_network().await {
        Ok(networks) => {
            println!("{} réseau(x) social(aux) trouvé(s) :", networks.len());
            for net in networks {
                println!(
                    "- [{}] {} -> {} (icône: {}, position: {})",
                    net.id, net.title, net.link, net.icon, net.position
                );
            }
        }
        Err(e) => eprintln!("Erreur lors de la récupération des réseaux sociaux : {:?}", e),
    }

    Ok(())
}
