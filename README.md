# Jupiter: A Minimal, Fast, and Customizable Shell Greeter

![Demo](./assets/usage.gif)

Welcome to the official Github repository of Jupiter, a minimal, blazing-fast, and customizable shell greeter. Whether you're a seasoned developer or just getting started with the command line, Jupiter aims to enhance your shell experience with its sleek design and extensive customization options.

## Overview

Jupiter is designed to provide a delightful greeting experience every time you open your shell. With its minimalistic approach, it delivers essential information while ensuring lightning-fast performance.

## Features

- **Minimalistic**: Keep distractions at bay with a clean and concise greeting.
- **Fast**: Enjoy instant access to your shell prompt without any lag.
- **Customizable**: Tailor Jupiter to suit your preferences with various configuration options.

## Installation

To get started with Jupiter, follow these simple steps. Make sure you have **Cargo** and **Rust** installed on your system.

### Step 1: Clone the GitHub Repository

```bash
git clone https://github.com/kadircelk/jupiter.git && cd jupiter
```

### Step 2: Build with Cargo

```bash
cargo build --release
```

### Step 3: Move the Binary

Move the compiled binary to a location within your `PATH` environment variable. For example, you can use `~/bin`.

```bash
mv target/release/jupiter $HOME/bin
```

### Step 4: Configure Your Shell
Add the following line to your shell profile (e.g., `.bashrc`, `.zshrc`, `$PROFILE` for PowerShell, etc.):
```
jupiter
```

### Additional Installation Guides

#### For Zsh Users

If you're using **Zsh** as your shell, you can add Jupiter to your configuration with:

```bash
echo "jupiter" >> ~/.zshrc
```

#### For Fish Shell Users

For **Fish** shell users, append the following line to your **Fish** configuration file (`~/.config/fish/config.fish`):

#### For Windows Users (PowerShell)

Windows users using **PowerShell** can add Jupiter to their profile by executing:

```powershell
Add-Content $PROFILE "`n jupiter"
```

## Contributing

We welcome contributions from the community! Whether you're a seasoned developer or new to open source, there are plenty of opportunities to get involved. Check out our [GitHub repository](https://github.com/kadircelk/jupiter) for more information on how you can contribute.

## License

Jupiter is licensed under the GNU General Public License v3.0. For details, please refer to [the license file](./LICENSE).

## Conclusion

Thank you for choosing Jupiter as your shell greeter. We hope it enhances your command-line experience and makes your interactions with the shell more enjoyable. If you have any questions or feedback, don't hesitate to reach out to us through our [GitHub repository](https://github.com/kadircelk/jupiter). Happy coding! 🚀
