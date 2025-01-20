use std::process::{Command, ExitStatus};
use std::fs;
use std::path::Path;

fn execute_steamcmd_depot(
    app_id: u64,
    depot_id: u64,
    manifest_id: Option<u64>, // Manifest-ID optional
    username: &str,
    password: &str,
    output_dir: &str,
) -> Result<ExitStatus, std::io::Error> {
    // Erstelle den Installationsordner, falls er nicht existiert
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Erstelle den SteamCMD-Befehl
    let mut command = Command::new("steamcmd");
    command
        .arg("+force_install_dir")
        .arg(output_dir) // Zielordner zuerst
        .arg("+login")
        .arg(username)
        .arg(password)
        .arg("+download_depot")
        .arg(app_id.to_string())
        .arg(depot_id.to_string());

    // Manifest hinzufügen, wenn angegeben
    if let Some(manifest) = manifest_id {
        command.arg(manifest.to_string());
    }

    command.arg("+quit");

    println!(
        "Starte SteamCMD-Befehl für App {}, Depot {} mit Zielordner {}",
        app_id, depot_id, output_dir
    );

    let status = command.spawn()?.wait()?;

    if !status.success() {
        eprintln!(
            "SteamCMD-Fehler bei App {}, Depot {}. Überprüfe die Logs.",
            app_id, depot_id
        );
    }

    Ok(status)
}

fn main() {
    let username = "Dein Username";
    let password = "Dein Passwort";
    let output_dir = "./downloads";
//test
    let depots = vec![
        (730, 731, Some(8382545303357448663)),
        (730, 732, Some(6288681278692387627)),
    ];

    for (app_id, depot_id, manifest_id) in depots {
        match execute_steamcmd_depot(app_id, depot_id, manifest_id, username, password, output_dir) {
            Ok(status) if status.success() => {
                println!("Download abgeschlossen: App {}, Depot {}", app_id, depot_id);
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