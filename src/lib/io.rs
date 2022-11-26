use std::process::Command;

//TODO: Error handling
pub fn clone_to_dir(repo: &str, target_dir: &str, branch: Option<&str>) {
    let command_result = Command::new("git")
        .arg("clone")
        .arg(branch.map_or("", |_| "--branch"))
        .arg(branch.unwrap_or(""))
        .arg(repo)
        .arg(target_dir)
        .spawn();

    if let Ok(mut child) = command_result {
        let exit_status = child.wait();
        
        if let Ok(status) = exit_status {
            println!("Successfully cloned repo to dir: {} (Status: {})", target_dir, status);
        }
    }
}

pub fn copy_dir(source_dir: &str, target_dir: &str) {
    let command_result = Command::new("cp").arg("-R").arg(source_dir).arg(target_dir).spawn();
}

/**
 *
 * Creates a directory and its parents (if set)
 *
 */
pub fn create_dir(dir: &str, create_parents: bool) {
    
    let parents_arg = if create_parents { "-p" } else { "" };

    let command_result = Command::new("mkdir").arg(parents_arg).arg(dir);
}
