use simple_hash::*;

const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

fn main() {
    let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
    let frequency_count = word_frequency_counter(&words);

    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}
