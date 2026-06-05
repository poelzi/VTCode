pub const BRAILLE_SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn frame(tick: u64) -> &'static str {
    let index = (tick as usize) % BRAILLE_SPINNER_FRAMES.len();
    BRAILLE_SPINNER_FRAMES[index]
}
