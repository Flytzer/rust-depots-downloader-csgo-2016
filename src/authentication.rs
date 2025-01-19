use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct SteamAuthResponse {
    success: bool,
    session_id: Option<String>,
    steam_id: Option<String>,
}

pub async fn authenticate(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let payload = json!({
        "username": username,
        "password": password,
    });

    // Anfrage senden
    let response = client
        .post("https://steamcommunity.com/openid/login") // URL anpassen!
        .json(&payload)
        .send()
        .await?;

    // Prüfen, ob die Anfrage erfolgreich war
    if !response.status().is_success() {
        return Err(format!("HTTP Error: {}", response.status()).into());
    }

    // JSON-Antwort in Struktur parsen
    let auth_response: SteamAuthResponse = response.json().await?;
    println!("Parsed Response: {:?}", auth_response);

    // Session-ID prüfen und zurückgeben
    if let Some(session_id) = auth_response.session_id {
        Ok(session_id)
    } else {
        Err("Session ID not found in authentication response".into())
    }
}
