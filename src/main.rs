use std::process::{Command, ExitStatus};
use std::fs;
use std::path::Path;

fn execute_steamcmd(app_id: u64, depot_id: u64, manifest_id: u64, username: &str, password: &str, output_dir: &str) -> Result<ExitStatus, std::io::Error> {
    // Erstelle den Installationsordner, falls er nicht existiert
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // SteamCMD-Befehl mit den gewünschten Parametern zusammenstellen
    let steamcmd_command = format!(
        "+login {} {} +force_install_dir {} +app_update {} -validate +quit",
        username, password, output_dir, app_id
    );

    // Depot-spezifischen Download ergänzen
    let depot_command = format!(" +download_depot {} {} {}", app_id, depot_id, manifest_id);

    // Führe den SteamCMD-Prozess aus
    Command::new("steamcmd")
        .arg("+login")
        .arg(username)
        .arg(password)
        .arg("+force_install_dir")
        .arg(output_dir)
        .arg("+app_update")
        .arg(app_id.to_string())
        .arg("+validate")
        .arg(&depot_command)
        .arg("+quit")
        .spawn()?
        .wait()
}

fn main() {
    // Beispielparameter
    let app_id = 730; // App-ID (z.B. CS:GO)
    let depot_id = 731; // Depot-ID
    let manifest_id = 8382545303357448663; // Manifest-ID
    let username = "Dein Steam Username";
    let password = "Dein Steam Passwort";
    let output_dir = "./downloads";

    match execute_steamcmd(app_id, depot_id, manifest_id, username, password, output_dir) {
        Ok(status) if status.success() => {
            println!("Download erfolgreich abgeschlossen!");
        }
        Ok(_) => {
            eprintln!("Download fehlgeschlagen.");
        }
        Err(e) => {
            eprintln!("Fehler beim Ausführen von SteamCMD: {:?}", e);
        }
    }
}
