use console::Style;

/// Holds styling for additions and deletions for the Dracula theme.
pub struct Theme {
    pub addition: Style,
    pub deletion: Style,
}

impl Theme {
    /// Returns the default Dracula theme.
    pub fn dracula() -> Self {
        Theme {
            addition: Style::new().on_color256(10).black(),  // Bright green bg, black text
            deletion: Style::new().on_color256(9).black(),   // Bright red bg, black text
        }
    }
}
