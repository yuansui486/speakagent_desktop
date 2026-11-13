//! Join recognizer tokens and VAD segments without inserting spaces into Chinese.

pub fn is_cjk(ch: char) -> bool {
    matches!(ch as u32, 0x3400..=0x9FFF | 0xF900..=0xFAFF | 0x20000..=0x323AF)
}

pub fn append(out: &mut String, next: &str) {
    let next = next.trim();
    if next.is_empty() {
        return;
    }
    if let (Some(last), Some(first)) = (out.chars().next_back(), next.chars().next()) {
        let closing = "，。！？；：、）》】」』,.!?;:%)]}".contains(first);
        let opening = "（《【「『([{".contains(last);
        if !last.is_whitespace() && !is_cjk(last) && !is_cjk(first) && !closing && !opening {
            out.push(' ');
        }
    }
    out.push_str(next);
}

pub fn join<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::new();
    for part in parts {
        append(&mut out, part);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chinese_segments_and_punctuation() {
        assert_eq!(
            join(["今天", "天气", "很好", "。", "我们", "出发", "！"]),
            "今天天气很好。我们出发！"
        );
    }

    #[test]
    fn english_and_russian_word_boundaries() {
        assert_eq!(
            join(["Hello", "world", ",", "again", "!"]),
            "Hello world, again!"
        );
        assert_eq!(join(["Привет", "мир", "!"]), "Привет мир!");
    }

    #[test]
    fn mixed_language_preserves_words_and_internal_spaces() {
        assert_eq!(
            join(["使用", "Visual Studio", "进行", "Rust", "开发"]),
            "使用Visual Studio进行Rust开发"
        );
        assert_eq!(join(["", "  ", "你好。", "下一句"]), "你好。下一句");
        assert_eq!(join(["（", "中文", "）"]), "（中文）");
    }
}
