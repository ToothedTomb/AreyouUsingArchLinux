use sysinfo::{System, SystemExt};
use termion::{color, style};

fn main() {
    // Create a System object to fetch system information
    let mut sys = System::new_all();
    sys.refresh_all();

    // Fetching information
    let os_name = sys.name().unwrap_or_else(|| "Unknown".to_string());

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
    let is_arch = os_name.to_lowercase().contains("arch");

    // Displaying information
    println!("{}{}Are You Using Arch Linux?{}", color::Fg(color::Cyan), style::Bold, style::Reset);
    println!("{}", arch_logo);

    let message = if is_arch {
        "Congratulations, you are running Arch Linux!"
    } else {
        "No, you are not using Arch Linux!"
    };

    println!("{}{}{}{}", color::Fg(color::Cyan), style::Bold, message, style::Reset);
}
