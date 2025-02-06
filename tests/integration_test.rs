use test_helper::{PrivatePrompt, PublicPrompt};

#[test]
fn test_public_fields() {
    let public_prompt = PublicPrompt {
        message: String::from("Test message"),
        level: String::from("info"),
        word_count: 2,
        time_remaining: 60,
    };

    // Public fields'a erişim
    assert_eq!(public_prompt.message, "Test message");
    assert_eq!(public_prompt.level, "info");
    assert_eq!(public_prompt.word_count, 2);
    assert_eq!(public_prompt.time_remaining, 60);
}
/* These tests are not compile! Because of that private fields!
#[test]
fn test_private_fields_should_fail() {
    let private_prompt = PrivatePrompt {
        message: String::from("Test message"), // Bu satır compile olmamalı
        level: String::from("info"),           // Bu satır compile olmamalı
        word_count: 2,                         // Bu satır compile olmamalı
        time_remaining: 60,                    // Bu satır compile olmamalı
    };

    // Bu satırlar compile olmamalı - private fields
    assert_eq!(private_prompt.message, "Test message");
    assert_eq!(private_prompt.level, "info");
    assert_eq!(private_prompt.word_count, 2);
    assert_eq!(private_prompt.time_remaining, 60);
}

*/
