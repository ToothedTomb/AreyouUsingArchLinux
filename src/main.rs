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


    // Displaying information
    println!("{}{}{}", color::Fg(color::Cyan), style::Bold, "Are You Using Arch Linux?");
    println!("{}", arch_logo);

    if os_name.contains("Arch"){
      println!("{}{}{}",
               color::Fg(color::Cyan), style::Bold, "Congratulations, you are running Arch Linux!");
  } else {
      println!("{}{}{}",
               color::Fg(color::Cyan), style::Bold, "No, you are not using Arch Linux!");
  }
}
