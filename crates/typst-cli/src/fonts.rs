use crate::args::FontsCommand;
use std::io::Write;
use typst::text::FontVariant;
use typst_kit::fonts::Fonts;
/// Execute a font listing command.
pub fn fonts(command: &FontsCommand) {
    let fonts = Fonts::searcher()
        .include_system_fonts(!command.font_args.ignore_system_fonts)
        .search_with(&command.font_args.font_paths);
    let mut out = crate::terminal::out();
    for (name, infos) in fonts.book.families() {
        writeln!(out, "{name}:").unwrap();
        if command.variants {
            for info in infos {
                let FontVariant { style, weight, stretch } = info.variant;
                writeln!(
                    out,
                    "- Style: {style:?}, Weight: {weight:?}, Stretch: {stretch:?}"
                )
                .unwrap();
            }
        }
    }
}
