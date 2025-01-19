use reqwest::Client;

pub async fn get_depot_info(app_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.steampowered.com/ISteamApps/GetAppDepots/v1/?key=YOUR_API_KEY&appid={}", app_id);

    let response = client.get(&url).send().await?.text().await?;
    println!("Depot Info: {}", response);

    Ok(())
}
