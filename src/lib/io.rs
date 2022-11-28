use std::process::{Command, ExitStatus};

//TODO: Error handling
pub fn clone_to_dir(repo: &str, target_dir: &str, branch: Option<&str>) {
    let mut command_result = Command::new("git");

    command_result.arg("clone");

    if let Some(b) = branch {
        command_result.args(["--branch", b]);
    }

    command_result.arg(repo).arg(target_dir);

    if let Ok(mut child) = command_result.spawn() {
        let exit_status = child.wait();

        if let Ok(status) = exit_status {
            if status.success() {
                println!(
                    "Successfully cloned repo to dir: {} (Status: {})",
                    target_dir, status
                );
            } else {
                println!("Cloning not successful");
            }
        }
    }
}

pub fn copy_all_files_to_dir(source_dir: &str, target_dir: &str) {
    copy_dir(&format!("{}/.", source_dir), &format!("{}/", target_dir));
}

pub fn copy_dir(source_dir: &str, target_dir: &str) {
    let mut command = Command::new("cp");
    command.arg("-r").arg(source_dir).arg(target_dir);

    if let Ok(mut child) = command.spawn() {
        let exit_status = child.wait();

        if let Ok(status) = exit_status {
            if status.success() {
                println!(
                    "Successfully copied source dir {} to dir: {} (Status: {})",
                    source_dir, target_dir, status
                );
            } else {
                println!("Copying not successful");
            }
        }
    }
}

/**
 *
 * Creates a directory and its parents (if set)
 *
 */
pub fn create_dir(dir: &str, create_parents: bool) {
    let parents_arg = if create_parents { "-p" } else { "" };

    let mut command = Command::new("mkdir");
    command.arg(parents_arg).arg(dir);

    if let Ok(mut child) = command.spawn() {
        let exit_status = child.wait();

        if let Ok(status) = exit_status {
            if status.success() {
                println!(
                    "Successfully created dir {} {}",
                    &dir, if create_parents {"(and respective parents)"} else {""}
                );
            } else {
                println!("Cloning not successful");
            }
        }
    }
}