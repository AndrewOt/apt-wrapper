mod command;
mod file;

use clap::{Parser, Subcommand};
use command::{install_command, remove_command};
use file::{add_package_to_manifest, get_manifest_contents, remove_package_manifest};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct AptWrapper {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Install { package_list: Vec<String> },
    Remove { package_list: Vec<String> },
    Restore,
}

pub fn entry() {
    operate_on_command(AptWrapper::parse());
}

fn operate_on_command(clap_wrapper: AptWrapper) {
    match clap_wrapper.command {
        Some(Commands::Install { package_list }) => {
            handle_install_command(package_list);
        }
        Some(Commands::Remove { package_list }) => {
            handle_remove_command(package_list);
        }
        Some(Commands::Restore) => handle_restore_command(),
        None => eprintln!("Only supports install, uninstall, and restore."),
    }
}

fn handle_install_command(packages: Vec<String>) {
    let install_result = install_command(&packages);
    match install_result {
        Some(true) => {
            let write_result = add_package_to_manifest(&packages);
            match write_result {
                Some(result) => match result {
                    Ok(_) => {
                        println!(
                            "Successfully updated manifest. Added package{}: {}",
                            if packages.len() > 1 { "s" } else { "" },
                            packages.join(", ")
                        );
                    }
                    Err(e) => eprintln!("Failed to write to the manifest. Error: {e}"),
                },
                None => eprintln!("There was a problem finding your home path."),
            }
        }
        _ => eprintln!("There was a error executing the install command. See output above."),
    }
}

fn handle_remove_command(packages: Vec<String>) {
    let remove_result = remove_command(&packages);
    match remove_result {
        Some(true) => {
            let write_result = remove_package_manifest(&packages);
            match write_result {
                Some(result) => match result {
                    Ok(_) => println!("Successfully updated manifest."),
                    Err(e) => eprintln!("Failed to write to the manifest. Error: {e}"),
                },
                None => eprintln!("There was a problem finding your home path"),
            }
        }
        _ => eprintln!("There was an error executing the remove command. See the output above."),
    }
}

fn handle_restore_command() {
    let manifest_packages_result = get_manifest_contents();
    match manifest_packages_result {
        Some(packages) => {
            let install_result = install_command(&packages.split(" ").map(String::from).collect());
            match install_result {
                Some(true) => println!("Succesfully installed packages from the manifest."),
                _ => eprintln!("There was an error executing the install command during restore. See output above.")
            }
        }
        _ => eprintln!(
            "There was a problem getting the packages from the manifest.
            Is the file in your home directory?"
        ),
    }
}
