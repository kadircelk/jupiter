# Jupiter Configuration Documentation

This documentation provides information about the configuration file of Jupiter shell greeter and where the configuration file will be located. It also explains how to create and where to place the configuration file for each operating system.

## Configuration File Format

The configuration file of Jupiter should be in JSON format. This configuration file defines the content to be displayed in the left and right sections of the shell greeter, as well as the size of the gap between these sections.

An example configuration file looks like this:

```json
{
    "left": ["Hello, {user}"],
    "right": ["{time}"],
    "gap": 5
}
```

- **"left"**: An array containing the content to be displayed in the left section of the shell greeter.
- **"right"**: An array containing the content to be displayed in the right section of the shell greeter.
- **"gap"**: An integer specifying the size of the gap between the left, right content and window border.

## Location of Configuration File

The configuration file can be located in a different location for each operating system. Below are instructions on where to place the configuration file for each operating system.

### Linux and macOS

Place the configuration file in the following location:

```bash
$HOME/.config/jupiter.json
```

### Windows

Place the configuration file in the following location:

```shell
%APPDATA%\jupiter.json
```

## Sample Configuration
```json
{
    "left": ["  {hour}:{minute}", "-", "  {day}.{month}.{year}"],
    "right": ["  {user}", "-", "󰄉  {uptime}"],
    "gap": 2
}
```

