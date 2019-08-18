use hangman;

#[test]
fn test_empty_hidden_word() {
    let target = hangman::RoundProgress {
        hidden_word: Vec::new(),
        failed_attempts: Vec::new(),
        status: hangman::RoundStatus::Ongoing,
        points: 0,
    };
    assert_eq!(hangman::get_hidden_word(&target), String::from(""));
}
