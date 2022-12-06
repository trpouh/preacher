use std::process::Command;

#[cfg(test)]
mod tests {
    use std::{process::{Command, Stdio}, io::stdin};

    use super::spawn_and_map_to_res;

    #[test]
    fn test_success() {
        let mut command = Command::new("true");

        let res = spawn_and_map_to_res(&mut command);

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_success_capture_output() {
        let mut command = Command::new("echo");
        command.arg("hallo welt");

        let res = spawn_and_map_to_res(&mut command);

        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "hallo welt\n");
    }

    #[test]
    fn test_failure() {
        let mut command = Command::new("false");

        let res = spawn_and_map_to_res(&mut command);

        assert_eq!(res.is_ok(), false);
    }
}

pub fn spawn_and_map_to_res(command: &mut Command) -> Result<String, String> {
    let cmd = command.output();

    if let Ok(out) = cmd {
        if out.status.success() {
            return Ok(String::from_utf8_lossy(&out.stdout).to_string());
        }

        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }

    Err(cmd.err().map(|err| err.to_string()).unwrap_or_default())
}
