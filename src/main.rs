use sysinfo::{System, SystemExt, ProcessorExt};
use termion::{color, style};
use nvml_wrapper::Nvml;

fn main() {
    // Create a System object to fetch system information
    let mut sys = System::new_all();
    sys.refresh_all();

    // Fetching information
    let os_name = sys.name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = sys.os_version().unwrap_or_else(|| "Unknown".to_string());
    // Initialize NVML and fetch GPU information
    let ArchLogo = r#"                    
        /\\
       /  \\
      /    \\
     /      \\
    /   ,,   \\
   /   |  |  -\\
  /_-''    ''-_\\
  
  "#;

let Areyou = match os_name.as_str() 
{
  "Arch" => r#"Congratulations you are running Arch Linux!"#,
  _ => r#"No, you are not using Arch Linux!"#
};

    // Displaying information

    println!("{}{}{}",
             color::Fg(color::Cyan), style::Bold, "Are You Using Arch Linux?");
    println!("{}",ArchLogo);

    println!("{}",Areyou);

}
