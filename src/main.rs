use sysinfo::{System, SystemExt};
use termion::{color, style};
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;

fn main() {
    // Create a System object to fetch system information
    let mut sys = System::new_all();
    sys.refresh_all();

    let arch_logo = r#"
        /\\
       /  \\
      /    \\
     /      \\
    /   ,,   \\
   /   |  |  -\\
  /_-''    ''-_\\
  
  "#;

    // Determine if the OS is Arch Linux
    let is_arch_linux = is_arch_linux();

    // Displaying information
    println!("{}{}{}", color::Fg(color::Cyan), style::Bold, "Are You Using Arch Linux?");
    println!("{}", arch_logo);

    if is_arch_linux {
        println!("{}{}{}",
                 color::Fg(color::Cyan), style::Bold, "Congratulations, you are running Arch Linux!");
    } else {
        println!("{}{}{}",
                 color::Fg(color::Cyan), style::Bold, "No, you are not using Arch Linux!");
    }
}

// Function to check if the OS is Arch Linux
fn is_arch_linux() -> bool {
    if check_os_release() && check_pacman() {
        return true;
    }
    false
}

// Check the /etc/os-release file for Arch Linux indicators
fn check_os_release() -> bool {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("ID=arch") || line.contains("ID_LIKE=arch") {
                    return true;
                }
            }
        }
    }
    false
}

// Check for the presence of the pacman package manager
fn check_pacman() -> bool {
    if let Ok(output) = Command::new("pacman").arg("--version").output() {
        return output.status.success();
    }
    false
}
