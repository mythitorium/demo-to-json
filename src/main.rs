pub mod parser;
use native_dialog::DialogBuilder;
use std::{io::Write, path::PathBuf};

fn main() {
    //let args: Vec<String> = env::args().collect();

    // Find tf2 installation
    let custom_set_location;
    if let Ok(tf2_path) = find_tf2() {

        custom_set_location = tf2_path.to_str().unwrap().to_string();
    } else {
        custom_set_location = "~/Desktop".to_string();
    }
    
    // File dialog
    let path = DialogBuilder::file()
        .set_location(&custom_set_location)
        .add_filter("Demo File", &["dem"])
        .open_single_file()
        .show()
        .unwrap();

    // Processing 
    if let Some(mut path) = path {
        let path_str = path.to_str().unwrap();

        println!("Parsing demo...");

        let parser = parser::Parser::new(path_str);

        println!("Converting into json format...");

        let json_string = serde_json::to_string(&parser).unwrap_or("".to_string());

        println!("Writing to file...");

        // Convert the input path to the output path (ie change from dem to json)
        path.set_extension("json");
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();

        println!("Writing complete");
        
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}


// Find tf2
fn find_tf2() -> Result<PathBuf, steamlocate::Error> {
    let steam_dir = steamlocate::SteamDir::locate()?;

    for library in steam_dir.libraries()? {
        let library = library?;
        if let Some(app) = library.app(440) {
            return Ok(library.path().join("steamapps").join("common").join(app?.install_dir).join("tf"));
        }
    }

    return Err(steamlocate::Error::FailedLocate(steamlocate::error::LocateError::Unsupported));
}
