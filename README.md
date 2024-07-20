
# AreYouUsingArchLinux

`AreYouUsingArchLinux` is a simple Rust-based tool that determines whether your system is running Arch Linux. It performs checks to identify if the operating system is Arch Linux and provides a visual confirmation with ASCII art and colored terminal output.

## Features

- **OS Detection:** Checks if the system is Arch Linux by examining both the `/etc/os-release` file and the presence of the `pacman` package manager.
- **Visual Feedback:** Displays a custom ASCII logo and colored terminal messages to make the output more engaging.

## Screenshot

![Screenshot_20240717_195619](https://github.com/user-attachments/assets/06cea7f2-3d4b-4fbf-8171-0c9e8214cefd)
![Screenshot_20240717_205740](https://github.com/user-attachments/assets/4fc25372-e565-4203-829a-91826de26d3e)

## How It Works

1. **System Information Retrieval:** Utilizes the `sysinfo` crate to gather system information.
2. **Arch Linux Detection:**
   - **OS Release File:** Reads and parses the `/etc/os-release` file to check for Arch Linux indicators (`ID=arch` or `ID_LIKE=arch`).
   - **Pacman Presence:** Checks for the `pacman` package manager to further verify Arch Linux.
3. **Output:** Prints a colored ASCII logo and a message indicating whether the system is Arch Linux or not.

## Installation

1. **Clone the Repository:**
   ```sh
   git clone https://github.com/your-username/AreYouUsingArchLinux.git
   ```

2. **Navigate to the Project Directory:**
   ```sh
   cd AreYouUsingArchLinux
   ```

3. **Build and Run:**
   Ensure you have Rust installed. Then run:
   ```sh
   cargo run
   ```

## Code Overview

### Key Functions

- **`main`**: Initializes the system object, checks if the OS is Arch Linux, and prints the result with colored output.
- **`is_arch_linux`**: Combines results from `check_os_release` and `check_pacman` to determine if the system is Arch Linux.
- **`check_os_release`**: Reads and parses `/etc/os-release` to identify Arch Linux indicators.
- **`check_pacman`**: Checks if `pacman` is installed and accessible.

### Example Output

- **Arch Linux Detected:**
  ```
  Are You Using Arch Linux?
  [ASCII Logo]
  Congratulations, you are running Arch Linux!
  ```

- **Not Arch Linux:**
  ```
  Are You Using Arch Linux?
  [ASCII Logo]
  No, you are not using Arch Linux!
  ```

## Notes

- The detection method relies on both the presence of Arch-specific identifiers in `/etc/os-release` and the `pacman` package manager. However, it’s still possible to trick the detection by modifying these files or altering system configurations.
