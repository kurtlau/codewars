fn solution(word: &str, ending: &str) -> bool {
    if word.len() < ending.len() {
        false
    } else {
        word[word.len() - ending.len()..].eq(ending)
    }
}
