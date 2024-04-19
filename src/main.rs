use std::io::stdout;

mod editor;
use editor::Editor;

mod terminal;
pub use terminal::Terminal;

fn main() {
    let editor = Editor::default().run();
}
