use crate::config::{resolve_config, Config};
use crate::style::{resolve_string, get_term_size};

/// Calculates the padding required to center-align the left and right content
/// within the terminal window.
///
/// ## Arguments
///
/// * `left_width` - The width of the left content in terms of character count.
/// * `right_width` - The width of the right content in terms of character count.
/// * `gap` - The gap between the left and right content.
///
/// ## Returns
///
/// A tuple `(usize, usize)` representing the padding for left and right content.
pub fn calculate_padding(left_width: usize, right_width: usize, gap: usize) -> (usize, usize) {
    let term_width = get_term_size().0 as usize;

    // Calculate the total width including content and gaps.
    let total_width = left_width + right_width + gap * 2;

    // Calculate padding needed for center alignment.
    let padding = if total_width >= term_width {
        0
    } else {
        term_width - total_width
    };

    return (padding, gap);
}

/// Main function to execute the program.
pub fn run() {
    let config: Config = resolve_config();

    let left_content = resolve_string(&config.left.join(" "));
    let right_content = resolve_string(&config.right.join(" "));

    let left_width = left_content.chars().count();
    let right_width = right_content.chars().count();

    // Calculate padding for center alignment.
    let (padding, gap) = calculate_padding(left_width, right_width, config.gap);

    // Generate padding spaces.
    let padding = " ".repeat(padding);

    println!();
    println!("{}{}{}{}{}", " ".repeat(gap), left_content, padding, right_content, " ".repeat(gap));
}
