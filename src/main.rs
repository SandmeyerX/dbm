use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{self, Write};


fn close_bluetooth() -> Result<(), Box<dyn std::error::Error>> {
    // Verify that the script exists
    let script_path = r".\src\blt.ps1";
    if !Path::new(script_path).exists() {
        return Err(format!("ps1 file {} does not exist", script_path).into());
    }

    // Execute Command
    let output = Command::new("powershell")
        .args(&["-Command", &format!("{} -BluetoothStatus Off", script_path)])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    // Process Output
    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        io::stderr().write_all(&output.stderr)?;
        return Err(format!("status code: {}", output.status).into());
    }
    Ok(())
}


fn main() {
    // TODO
    println!("{:?}", close_bluetooth());
}
