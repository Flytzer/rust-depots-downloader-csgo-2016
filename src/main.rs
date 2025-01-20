use std::process::{Command, ExitStatus};
use std::fs;
use std::path::Path;

fn execute_steamcmd_depot(
    app_id: u64,
    depot_id: u64,
    manifest_id: u64,
    username: &str,
    password: &str,
    output_dir: &str,
) -> Result<ExitStatus, std::io::Error> {
    // Erstelle den Installationsordner, falls er nicht existiert
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Führe den SteamCMD-Befehl aus
    Command::new("steamcmd")
        .arg("+login")
        .arg(username)
        .arg(password)
        .arg("+force_install_dir")
        .arg(output_dir)
        .arg("+download_depot")
        .arg(app_id.to_string())
        .arg(depot_id.to_string())
        .arg(manifest_id.to_string())
        .arg("+quit")
        .spawn()?
        .wait()
}

fn main() {
    // Hardcodierte Zugangsdaten und Zielordner
    let username = "Dein Steam Username";
    let password = "Dein Steam Passwort";
    let output_dir = "./downloads";

    // Liste der Depots, die heruntergeladen werden sollen
    let depots = vec![
        (730, 731, 8382545303357448663), // App 730, Depot 731
        (730, 732, 6288681278692387627), // App 730, Depot 732
    ];

    for (app_id, depot_id, manifest_id) in depots {
        println!(
            "Starte Download: App {}, Depot {}, Manifest {} in '{}'",
            app_id, depot_id, manifest_id, output_dir
        );

        match execute_steamcmd_depot(app_id, depot_id, manifest_id, username, password, output_dir) {
            Ok(status) if status.success() => {
                println!(
                    "Download abgeschlossen: App {}, Depot {} in '{}'",
                    app_id, depot_id, output_dir
                );
            }
            Ok(_) => {
                eprintln!("Download fehlgeschlagen: App {}, Depot {}.", app_id, depot_id);
            }
            Err(e) => {
                eprintln!(
                    "Fehler beim Ausführen von SteamCMD: App {}, Depot {}. Fehler: {:?}",
                    app_id, depot_id, e
                );
            }
        }
    }

    println!("Alle Downloads abgeschlossen!");
}
