use std::io::Write;

use ecow::eco_format;
use typst::{diag::StrResult, text::FontVariant};
use typst_kit::fonts::Fonts;

use crate::args::FontsCommand;

/// Execute a font listing command.
pub fn fonts(command: &FontsCommand) -> StrResult<()> {
    let fonts = Fonts::searcher()
        .include_system_fonts(!command.font_args.ignore_system_fonts)
        .search_with(&command.font_args.font_paths);
    let mut out = std::io::stdout();
    for (name, infos) in fonts.book.families() {
        writeln!(out, "{name}:")
            .map_err(|err| eco_format!("failed to print fonts ({err})"))?;
        if command.variants {
            for info in infos {
                let FontVariant { style, weight, stretch } = info.variant;
                writeln!(
                    out,
                    "- Style: {style:?}, Weight: {weight:?}, Stretch: {stretch:?}"
                )
                .map_err(|err| eco_format!("failed to print fonts ({err})"))?;
            }
        }
    }
    Ok(())
}
