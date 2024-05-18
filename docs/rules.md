# String Formatting Rules for Jupiter

This documentation explains how users can format strings using rules to create beautiful UIs with the Jupiter shell greeter. Each rule is explained separately to provide detailed guidance on formatting strings effectively.

## Overview

Jupiter allows users to customize the appearance of their shell greeter by resolving placeholders in strings with actual system information. These placeholders, combined with shell commands, enable users to create dynamic and informative UIs for their terminal.

## Placeholder Rules

### Basic Placeholders
{user}

    Description: Resolves to the current user's username.
    Usage: Include {user} in your string to display the current username.

{hostname}

    Description: Resolves to the hostname of the system.
    Usage: Include {hostname} in your string to display the hostname of the system.

{pwd}

    Description: Resolves to the current working directory.
    Usage: Include {pwd} in your string to display the current working directory.

{time}

    Description: Resolves to the current time in HH:MM:SS format.
    Usage: Include {time} in your string to display the current time.

### Date and Time Placeholders
{hour}, {minute}, {second}

    Description: Resolves to the current hour, minute, or second, respectively.
    Usage: Include {hour}, {minute}, or {second} in your string to display the current hour, minute, or second.

{year}, {month}, {day}

    Description: Resolves to the current year, month, or day, respectively.
    Usage: Include {year}, {month}, or {day} in your string to display the current year, month, or day.

{day_of_week}, {day_of_month}, {day_of_year}

    Description: Resolves to the current day of the week, day of the month, or day of the year, respectively.
    Usage: Include {day_of_week}, {day_of_month}, or {day_of_year} in your string to display the corresponding information.

{week_of_year}, {month_of_year}, {year_of_century}

    Description: Resolves to the current week of the year, month of the year, or year of the century, respectively.
    Usage: Include {week_of_year}, {month_of_year}, or {year_of_century} in your string to display the corresponding information.

### System Information Placeholders
{cpu_usage}

    Description: Resolves to the current CPU usage percentage.
    Usage: Include {cpu_usage} in your string to display the current CPU usage percentage.

{memory_usage}

    Description: Resolves to the current memory usage percentage.
    Usage: Include {memory_usage} in your string to display the current memory usage percentage.

{disk_usage}

    Description: Resolves to the current disk usage percentage.
    Usage: Include {disk_usage} in your string to display the current disk usage percentage.

{uptime}

    Description: Resolves to the system uptime.
    Usage: Include {uptime} in your string to display the system uptime.

{load_average}

    Description: Resolves to the system load average.
    Usage: Include {load_average} in your string to display the system load average.

{logged_in_users}

    Description: Resolves to the list of logged-in users.
    Usage: Include {logged_in_users} in your string to display the list of logged-in users.

{network_info}

    Description: Resolves to the network information of the system.
    Usage: Include {network_info} in your string to display the network information.

{process_count}

    Description: Resolves to the total number of processes.
    Usage: Include {process_count} in your string to display the total number of processes.

{public_ip}, {private_ip}

    Description: Resolves to the public or private IP address of the system, respectively.
    Usage: Include {public_ip} or {private_ip} in your string to display the corresponding IP address.

{kernel_version}, {system_architecture}

    Description: Resolves to the kernel version or system architecture, respectively.
    Usage: Include {kernel_version} or {system_architecture} in your string to display the corresponding information.

## Usage Example

To use these placeholders effectively, incorporate them into your strings using curly braces {}. For example:

```json
{
    "left": ["Welcome, {user}! Today is {day_of_week}, {month} {day}, {year}. Current time is {time}."],
    "right": [""],
    "gap": 2
}
```

This will produce an output like:

```
Welcome, john! Today is Monday, May 19, 2024. Current time is 12:30:45.
```

By combining different placeholders and string literals, users can create custom and informative UIs for their shell greeter in Jupiter.