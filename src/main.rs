pub mod parser;
use native_dialog::DialogBuilder;
use std::{env, io::Write, path::PathBuf, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut demo_paths: Vec<PathBuf> = Vec::new();
    let mut abortion_reason = "No file was provided or found";

    //
    // Acquire paths
    //

    // If an arg was provided
    if args.len() > 1 {
        let arg_path = PathBuf::from(args[1].clone());

        // If the path actually points somewhere
        if arg_path.exists() {
            // If it's a folder, then we're doing a bulk filter and process of all child files
            if arg_path.is_dir() {
                let entries = fs::read_dir(arg_path).unwrap();
                for boxed_entry in entries {
                    if let Ok(entry) = boxed_entry {           // It could be another folder
                        if entry.path().extension().unwrap() == "dem" && entry.path().is_file() {  demo_paths.push(entry.path());  }
                    }
                }
            // It's a file
            } else {
                // Only add if it's .dem
                if arg_path.extension().unwrap() == "dem"   {  demo_paths.push(arg_path);  } 
                else                                        {  abortion_reason = "Provided file wasn't a .dem"; }
            }
        } else {
            abortion_reason = "Provided path doesn't exist";
        }

    // No arg, request a file via file dialog
    } else {
        // Find tf2 installation
        let custom_set_location;
        if let Ok(tf2_path) = attempt_to_find_tf2() {
            custom_set_location = tf2_path.to_str().unwrap().to_string();
        } else {
            custom_set_location = "~/Desktop".to_string();
        }

        // Open file dialog. Process will hang until this closes
        let path = DialogBuilder::file()
            .set_location(&custom_set_location)
            .add_filter("Demo File", &["dem"])
            .open_single_file()
            .show()
            .unwrap(); 

        if let Some(path) = path { demo_paths.push(path); }
    }

    //
    // Process acquired demo paths, if any
    //

    // No file was provided
    if demo_paths.len() == 0 {
        println!("{}, process aborted", abortion_reason);
    // One file was provided
    } else if demo_paths.len() == 1 {
        process_demo(demo_paths[0].clone());
    // Multiple files have been provided
    } else {
        let mut count = 1;
        let amount = demo_paths.len();
        println!("Given directory contains {} .dem files", amount);
        for demo_path in demo_paths {
            println!("");
            println!("{}/{} '{}'", count, amount, demo_path.file_name().unwrap().to_str().unwrap());
            process_demo(demo_path);
            count += 1;
        }
    }
}


// Process a demo file.
fn process_demo(mut path: PathBuf) {
    let path_str = path.to_str().unwrap();

    println!("Parsing demo...");
    
    let parser = parser::Parser::new(path_str);

    println!("Converting into json format...");

    let json_string = serde_json::to_string(&parser).unwrap_or("".to_string());
    println!("Writing to file...");

    // Convert the input path to the output path (ie change from dem to json)
    path.set_extension("json");

    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    println!("Writing complete");

    println!("Saved to '{}'", path.to_str().unwrap_or(""));
}


// Find tf2
fn attempt_to_find_tf2() -> Result<PathBuf, steamlocate::Error> {
    let steam_dir = steamlocate::SteamDir::locate()?;

    for library in steam_dir.libraries()? {
        let library = library?;
        if let Some(app) = library.app(440) {
            return Ok(library.path().join("steamapps").join("common").join(app?.install_dir).join("tf"));
        }
    }

    return Err(steamlocate::Error::FailedLocate(steamlocate::error::LocateError::Unsupported));
}
