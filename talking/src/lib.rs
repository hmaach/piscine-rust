enum TalkingType {
    Yell,
    Ask,
    YellAsk,
    Nothing,
    Else,
}

pub fn talking(text: &str) -> &str {
    let talking_type = match text {
        s if s.chars().count() == 0 => TalkingType::Nothing,
        s if s.chars().last().unwrap_or(' ') == '?' => {
            if text.chars().any(|ch| ch.is_ascii_lowercase()) {
                TalkingType::Ask
            } else {
                TalkingType::YellAsk
            }
        }
        _ => {
            if text.chars().any(|ch| ch.is_ascii_lowercase()) {
                TalkingType::Else
            } else {
                TalkingType::Yell
            }
        }
    };

    match talking_type {
        TalkingType::Yell => "There is no need to yell, calm down!",
        TalkingType::Ask => "Sure.",
        TalkingType::YellAsk => "Quiet, I am thinking!",
        TalkingType::Nothing => "Just say something!",
        TalkingType::Else => "Interesting",
    }
}
