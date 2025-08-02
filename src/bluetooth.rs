// TODO: Customize ErrorKind

use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{self, Write};


pub trait BtDevice {
    fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Device;

#[derive(Default)]
pub struct WindowsBtDevice;

// TODO: Implement Linux and MacOS versions
#[derive(Default)]
pub struct LinuxBtDevice;

#[derive(Default)]
pub struct MacOSBtDevice;

#[derive(Default)]
pub struct UnknownBtDevice;


impl Device {
    #[cfg(target_os = "windows")]
    pub fn new() -> WindowsBtDevice {
        WindowsBtDevice::default()
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> LinuxBtDevice {
        LinuxBtDevice::default()    
    }

    #[cfg(target_os = "macos")]
    pub fn new() -> MacOSBtDevice {
        MacOSBtDevice::default()
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    pub fn new() -> UnknownBtDevice {
        UnknownBtDevice::default()
    }
}

impl WindowsBtDevice  {
    fn verify_script_exists<P: AsRef<Path>>(&self, script_path: P) -> Result<(), Box<dyn std::error::Error>> {
        let script_path = script_path.as_ref();
        if !script_path.exists() {
            return Err(
                format!("ps1 file {} does not exist", script_path.display())
                    .into()
            );
        }
        Ok(())
    }
}

impl BtDevice for WindowsBtDevice {
    fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Verify that the script exists
        let script_path = r".\src\blt.ps1";
        self.verify_script_exists(script_path)?;

        // Execute Command
        let output = Command::new("powershell")
            // XXX: The hardcoded relative path r".\src\blt.ps1" creates a magic string that 
            //      may not work correctly when the executable is run from different directories. 
            //      Consider making this configurable or using a more reliable path resolution method
            .args(&["-Command", &format!("{} -BluetoothStatus Off", script_path)])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        // Process Output
        if output.status.success() {
            io::stdout().write_all(&output.stdout)?;
        } else {
            io::stderr().write_all(&output.stderr)?;
            return Err(
                format!("status code: {}", output.status)
                    .into()
            );
        }
        Ok(())
    }
}
