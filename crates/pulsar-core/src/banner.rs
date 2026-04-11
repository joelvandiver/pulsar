/// The PULSAR banner, rendered as a dot-matrix display.
///
/// Uses `·` (U+00B7 middle dot) for background grid dots and
/// `●` (U+25CF black circle) for filled pixels, giving a
/// LED/punch-card readout aesthetic.
pub const BANNER: &str = concat!(
    "·●●●·  ·●··●  ·●···  ·●●●·  ··●··  ·●●●·\n",
    "·●··●  ·●··●  ·●···  ·●···  ·●·●·  ·●··●\n",
    "·●●●·  ·●··●  ·●···  ·●●●·  ·●●●·  ·●●●·\n",
    "·●···  ·●··●  ·●···  ····●  ·●·●·  ·●●··\n",
    "·●···  ·●●●·  ·●●●·  ·●●●·  ·●·●·  ·●··●",
);

/// The one-line project subtitle.
pub const SUBTITLE: &str =
    "Platform for Unified Learning through Systems Architecture in Rust";

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
        // 5 dot-matrix rows + 2 rules + 1 subtitle = 8 lines
        assert_eq!(banner().lines().count(), 8);
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
