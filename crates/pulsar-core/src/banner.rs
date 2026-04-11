/// The PULSAR banner, rendered as a dot-matrix display.
///
/// Uses spaces for the background and `●` (U+25CF) for filled pixels.
pub const BANNER: &str = concat!(
    " ●●●    ●  ●   ●      ●●●     ●     ●●● \n",
    " ●  ●   ●  ●   ●      ●      ● ●    ●  ●\n",
    " ●●●    ●  ●   ●      ●●●    ●●●    ●●● \n",
    " ●      ●  ●   ●         ●   ● ●    ●●  \n",
    " ●      ●●●    ●●●    ●●●    ● ●    ●  ●",
);

/// The two-line project subtitle.
pub const SUBTITLE: &str = concat!(
    "  Platform for Unified Learning \n",
    "    through Systems Architecture in Rust",
);

/// Returns the full startup banner: dot-matrix logo + subtitle.
pub fn banner() -> String {
    let width = 43; // width of the dot-matrix rows
    let rule = "─".repeat(width);
    format!("{BANNER}\n{rule}\n{SUBTITLE}\n{rule}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_contains_all_rows() {
        // 5 dot-matrix rows + 2 rules + 2 subtitle lines = 9 lines
        assert_eq!(banner().lines().count(), 9);
    }

    #[test]
    fn dot_matrix_rows_are_equal_width() {
        let rows: Vec<&str> = BANNER.lines().collect();
        assert_eq!(rows.len(), 5);
        let widths: Vec<usize> = rows.iter().map(|r| r.chars().count()).collect();
        let first = widths[0];
        assert!(widths.iter().all(|&w| w == first), "rows differ in width: {widths:?}");
    }
}
