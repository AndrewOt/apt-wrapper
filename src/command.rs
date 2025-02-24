use std::process::{Command, Output, Stdio};

pub fn install_command(packages: &Vec<String>) -> Option<bool> {
    let command_result = pass_command_to_system(&packages, "install");
    match command_result {
        Ok(_) => Some(true),
        _ => None,
    }
}

pub fn remove_command(packages: &Vec<String>) -> Option<bool> {
    let command_result = pass_command_to_system(&packages, "remove");
    match command_result {
        Ok(_) => Some(true),
        _ => None,
    }
}

fn pass_command_to_system(
    command_list: &Vec<String>,
    command: &str,
) -> Result<Output, std::io::Error> {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("apt"));
    args.push(command.to_string());
    args.append(command_list.to_owned().as_mut());

    let result = Command::new("sudo")
        .args(args)
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output();

    result
}
