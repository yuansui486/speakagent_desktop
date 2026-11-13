//! PDF export with embedded DejaVu and Noto Sans SC for multilingual text.

use genpdf::{elements, style, Document, Element, SimplePageDecorator};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfBlock {
    pub heading: Option<String>, // имя спикера (жирным)
    pub time: Option<String>,    // таймкод
    pub body: String,            // текст реплики / абзац
}

// genpdf only splits on ASCII spaces. Separate CJK glyph runs into styled
// pieces so continuous Chinese can wrap without adding spaces to exported text.
fn push_text(
    paragraph: &mut elements::Paragraph,
    text: &str,
    style: style::Style,
    cjk: genpdf::fonts::FontFamily<genpdf::fonts::Font>,
) {
    let mut start = 0;
    for (offset, ch) in text.char_indices() {
        if matches!(ch as u32, 0x2E80..=0x9FFF | 0xF900..=0xFAFF | 0x20000..=0x323AF | 0xFF00..=0xFFEF) {
            if start < offset {
                paragraph.push_styled(text[start..offset].to_owned(), style);
            }
            paragraph.push_styled(ch.to_string(), style.with_font_family(cjk));
            start = offset + ch.len_utf8();
        }
    }
    if start < text.len() {
        paragraph.push_styled(text[start..].to_owned(), style);
    }
}

fn paragraph(text: &str, cjk: genpdf::fonts::FontFamily<genpdf::fonts::Font>) -> elements::Paragraph {
    let mut paragraph = elements::Paragraph::default();
    push_text(&mut paragraph, text, style::Style::new(), cjk);
    paragraph
}

pub fn save_pdf(title: &str, blocks: &[PdfBlock], path: &str) -> Result<(), String> {
    // Keep the original Latin/Cyrillic face, bold headings and checklist glyphs.
    let regular = genpdf::fonts::FontData::new(
        include_bytes!("../../fonts/DejaVuSans.ttf").to_vec(), None,
    ).map_err(|e| e.to_string())?;
    let bold = genpdf::fonts::FontData::new(
        include_bytes!("../../fonts/DejaVuSans-Bold.ttf").to_vec(), None,
    ).map_err(|e| e.to_string())?;
    let family = genpdf::fonts::FontFamily {
        regular: regular.clone(), bold: bold.clone(), italic: regular, bold_italic: bold,
    };
    let mut doc = Document::new(family);
    // Static TrueType instances work offline without system font dependencies.
    let regular = genpdf::fonts::FontData::new(
        include_bytes!("../../fonts/NotoSansSC-Regular.ttf").to_vec(), None,
    ).map_err(|e| e.to_string())?;
    let bold = genpdf::fonts::FontData::new(
        include_bytes!("../../fonts/NotoSansSC-Bold.ttf").to_vec(), None,
    ).map_err(|e| e.to_string())?;
    let cjk = doc.add_font_family(genpdf::fonts::FontFamily {
        regular: regular.clone(), bold: bold.clone(), italic: regular, bold_italic: bold,
    });
    doc.set_title(title);
    let mut deco = SimplePageDecorator::new();
    deco.set_margins(15);
    doc.set_page_decorator(deco);

    doc.push(paragraph(title, cjk).styled(style::Style::new().bold().with_font_size(17)));
    doc.push(elements::Break::new(1.0));

    for b in blocks {
        if let Some(h) = &b.heading {
            let mut p = elements::Paragraph::default();
            push_text(&mut p, h, style::Style::new().bold(), cjk);
            if let Some(t) = &b.time {
                p.push_styled(
                    format!("   {t}"),
                    style::Style::new().with_color(style::Color::Rgb(140, 140, 140)),
                );
            }
            doc.push(p);
        }
        doc.push(paragraph(&b.body, cjk));
        doc.push(elements::Break::new(0.6));
    }

    doc.render_to_file(path).map_err(|e| e.to_string())
}
