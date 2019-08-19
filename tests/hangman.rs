use hangman;

#[test]
fn test_get_empty_random_list() {
    let target = hangman::get_list();

    // Assert non-empty
    assert_eq!(target.is_empty(), false);
}

#[test]
fn test_hide_empty_word() {
    let input = String::from("");
    let target = hangman::hide_word(&input);

    assert_eq!(target.is_empty(), true);
}

#[test]
fn test_hide_none_empty_word() {
    let input = String::from("test word");
    let target = hangman::hide_word(&input);

    // Assert non-empty
    assert_eq!(target.is_empty(), false);

    // Assert data matches
    let mut input_ch = input.chars();
    for target_ch in target {
        assert_eq!(target_ch.character, input_ch.next().unwrap());
        assert_eq!(target_ch.is_hidden, true);
    }
}

#[test]
fn test_get_empty_hidden_word() {
    let target = hangman::RoundProgress {
        hidden_word: Vec::new(),
        failed_attempts: Vec::new(),
        status: hangman::RoundStatus::Ongoing,
        points: 0,
    };
    assert_eq!(hangman::get_hidden_word(&target), String::from(""));
}

#[test]
fn test_get_non_empty_hidden_word() {
    let input = String::from("test word");
    let target = hangman::round_init(&input);

    // Assert non-empty
    assert_eq!(target.hidden_word.is_empty(), false);

    // Assert data matches
    let mut input_ch = input.chars();
    for target_ch in target.hidden_word {
        assert_eq!(target_ch.character, input_ch.next().unwrap());
        assert_eq!(target_ch.is_hidden, true);
    }
}

#[test]
fn test_evaluate_valid_user_input() {
    assert_eq!(hangman::evaluate_input(&"0"), '0');
    assert_eq!(hangman::evaluate_input(&"a"), 'a');
    assert_eq!(hangman::evaluate_input(&"A"), 'a');
    assert_eq!(hangman::evaluate_input(&"A "), 'a');
    assert_eq!(hangman::evaluate_input(&" A"), 'a');
    assert_eq!(hangman::evaluate_input(&" A "), 'a');

}

#[test]
fn test_evaluate_invalid_user_inputs() {
    assert_eq!(hangman::evaluate_input(&""), '!');
    assert_eq!(hangman::evaluate_input(&" "), '!');
    assert_eq!(hangman::evaluate_input(&"6"), '!');
    assert_eq!(hangman::evaluate_input(&"@"), '!');
    assert_eq!(hangman::evaluate_input(&"not char"), '!');
}
