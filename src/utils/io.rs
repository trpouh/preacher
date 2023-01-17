use std::process::Command;

use super::cmd;

//TODO: Error handling
//TODO: Use Command Result
pub fn clone_to_dir(repo: &str, target_dir: &str, branch: Option<&str>) -> Result<String, String> {
    debug!(
        "Trying to clone git repo {}:({})",
        repo,
        branch.unwrap_or("main")
    );

    let mut command_result = Command::new("git");

    command_result.arg("clone");

    if let Some(b) = branch {
        command_result.args(["--branch", b]);
    }

    command_result.arg(repo).arg(target_dir);

    let res = cmd::spawn_and_map_to_res(&mut command_result);

    match &res {
        Ok(ok) => debug!(
            "Successfully cloned repo to dir: {} (Status: {})",
            target_dir, ok
        ),
        Err(err) => error!("Cloning repo was not successful: {}", err),
    };

    res
}

pub struct CopyOptions<'a> {
    pub source_dir: &'a str,
    pub target_dir: &'a str,
    pub exclude: Option<Vec<&'a str>>,
    pub without_parent_folder: Option<bool>,
    pub ensure_target_exists: Option<bool>,
}

pub fn copy_dir(copy_options: &CopyOptions) {
    if copy_options.ensure_target_exists.unwrap_or(false) {
        create_dir(copy_options.target_dir, true);
    }

    let mut command = Command::new("rsync");

    let source_dir = if copy_options.without_parent_folder.unwrap_or(false) {
        format!("{}/.", copy_options.source_dir)
    } else {
        copy_options.source_dir.to_owned()
    };

    copy_options
        .exclude
        .as_ref()
        .unwrap_or(&Vec::default())
        .iter()
        .for_each(|dir| {
            command.args(["--exclude", dir]);
        });

    command
        .arg("-r")
        .arg(&source_dir)
        .arg(copy_options.target_dir);

    if let Ok(mut child) = command.spawn() {
        let exit_status = child.wait();

        if let Ok(status) = exit_status {
            if status.success() {
                debug!(
                    "Successfully copied source dir {} to dir: {} (Status: {})",
                    source_dir, copy_options.target_dir, status
                );
            } else {
                error!("Copying not successful");
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
                debug!(
                    "Successfully created dir {} {}",
                    &dir,
                    if create_parents {
                        "(and respective parents)"
                    } else {
                        ""
                    }
                );
            } else {
                error!("Cloning not successful");
            }
        }
    }
}
