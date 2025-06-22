enum TalkingType {
    Yell,
    Ask,
    YellAsk,
    Nothing,
    Else,
}

pub fn all_uppercase(s: &str) -> bool {
    let mut upper = 0;
    for ch in s.chars() {
        if ch >= 'A' && ch <= 'Z' {
            upper += 1;
        } else if ch >= 'a' && ch <= 'z' {
            return false;
        }
    }
    upper != 0
}

pub fn talking(text: &str) -> &str {
    let talking_type = match text {
        s if s.trim().chars().count() == 0 => TalkingType::Nothing,
        s if s.chars().last().unwrap_or(' ') == '?' => {
            if all_uppercase(text) {
                TalkingType::YellAsk
            } else {
                TalkingType::Ask
            }
        }
        _ => {
            if all_uppercase(text) {
                TalkingType::Yell
            } else {
                TalkingType::Else
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
