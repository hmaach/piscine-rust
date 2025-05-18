enum Answer {
    Yelling,
    YellingQuestion,
    NotYelling,
    Default,
}

pub fn talking(text: &str) -> &str {
    if text.to_string().trim().len() == 0 {
        return "Just say something!";
    }

    let talk_type: Answer;
    let mut is_yelling = true;
    let mut not_yelling = false;
    let mut is_question = false;
    let all_alphabitics = text.to_string().chars().any(|c| c.is_alphabetic());

    if !all_alphabitics {
        return "Sure.";
    }
    if text.to_string().chars().last().unwrap() == '?' {
        is_question = true;
    }

    for ch in text.to_string().chars() {
        if ch.is_lowercase() {
            is_yelling = false;
        } else if ch.is_uppercase() {
            not_yelling = true;
        }
    }

    if is_yelling && !is_question && all_alphabitics {
        talk_type = Answer::Yelling;
    } else if is_yelling && is_question && all_alphabitics {
        talk_type = Answer::YellingQuestion;
    } else if not_yelling && is_question {
        talk_type = Answer::NotYelling;
    } else {
        talk_type = Answer::Default;
    }

    match talk_type {
        Answer::Yelling => "There is no need to yell, calm down!",
        Answer::YellingQuestion => "Quiet, I am thinking!",
        Answer::NotYelling => "Sure.",
        Answer::Default => "Interesting",
    }
}
