use hangman;

#[test]
fn test_empty_hidden_word() {
    let target = hangman::RoundProgress {
        hidden_word: Vec::new(),
        status: hangman::RoundStatus::Ongoing,
        points: 0,
        failed_attempts: 0,
    };
    assert_eq!(hangman::get_hidden_word(&target), String::from(""));
}
