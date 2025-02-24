use home::home_dir;
use std::{fs, io::Result, path::PathBuf};

const MANIFEST_FILENAME: &str = "apt-manifest.txt";

pub fn add_package_to_manifest(packages: &Vec<String>) -> Option<Result<()>> {
    let user_home_directory = get_home_directory();
    let mut mutable_packages: Vec<String> = packages.clone();

    match user_home_directory {
        Some(home_path) => {
            let package_list_to_write: String;
            if fs::exists(&home_path).is_ok_and(|result| result == true) {
                let manifest_contents = fs::read_to_string(&home_path)
                    // We should always be able to read this if the home path exists.
                    .expect("Unable to read manifest file contents.");
                if manifest_contents == "" {
                    package_list_to_write = mutable_packages.join(" ").trim().to_string();
                } else {
                    let mut split_manifest: Vec<String> = manifest_contents
                        .split(" ")
                        .map(|s| s.to_string())
                        .collect();
                    for p in split_manifest.clone() {
                        let index = packages.iter().position(|item| p == *item);
                        match index {
                            Some(i) => {
                                mutable_packages.remove(i);
                            }
                            None => (),
                        }
                    }
                    split_manifest.append(&mut mutable_packages);
                    package_list_to_write = split_manifest.join(" ");
                }
            } else {
                package_list_to_write = mutable_packages.join(" ").trim().to_string();
            }

            let write_result = fs::write(&home_path, package_list_to_write);
            Some(write_result)
        }
        None => None,
    }
}

pub fn remove_package_manifest(packages: &Vec<String>) -> Option<Result<()>> {
    let user_home_directory = get_home_directory();

    match user_home_directory {
        Some(home_path) => {
            if fs::exists(&home_path).is_ok_and(|result| result == true) {
                let mut manifest_contents = fs::read_to_string(&home_path)
                    // We should always be able to read this if the home path exists.
                    .expect("Unable to read manifest file contents.")
                    .split(" ")
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>();
                for p in packages {
                    let index = manifest_contents.iter().position(|item| p == item);
                    match index {
                        Some(i) => {
                            manifest_contents.remove(i);
                        }
                        None => (),
                    }
                }
                let write_result = fs::write(&home_path, manifest_contents.join(" ").trim());
                Some(write_result)
            } else {
                eprintln!("Unable to locate manifest file. Updating manifest aborted.");
                None
            }
        }
        None => None,
    }
}

pub fn get_manifest_contents() -> Option<String> {
    let home_dir = get_home_directory();
    match home_dir {
        Some(home_path) => {
            if fs::exists(&home_path).is_ok_and(|result| result == true) {
                let content_result = fs::read_to_string(&home_path);
                match content_result {
                    Ok(content) => Some(content),
                    _ => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_home_directory() -> Option<PathBuf> {
    match home_dir() {
        Some(path) if path.try_exists().is_ok_and(|result| result == true) => {
            let mut home_directory = path;
            home_directory.push(MANIFEST_FILENAME);
            Some(home_directory)
        }
        _ => {
            eprintln!("Unable to get your home dir!");
            None
        }
    }
}
