use std::process::Command;
use std::io::Result;


fn shutdown() -> Result<()> {
    #[cfg(target_os = "windows")] {
        Command::new("shutdown").args(["/s", "/t", "0"]).status()?;
    }

    #[cfg(target_os = "linux")] {
        Command::new("shutdown").args(["-h", "now"]).status()?;
    }

    #[cfg(target_os = "macos")] {
        Command::new("sudo").args(["shutdown", "-h", "now"]).status()?;
    }

    Ok(())
}

fn main() {
    match shutdown() {
        Ok(()) => println!("изи"),
        Err(e) => eprintln!("failed to execute command, {}", e),
    }
}
