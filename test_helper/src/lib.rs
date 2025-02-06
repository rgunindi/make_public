use make_public::make_public;

// Test 1: make_public olmadan (private fields)
#[derive(Debug)]
pub struct PrivatePrompt {
    message: String,
    level: String,
    word_count: usize,
    time_remaining: i32,
}

// Test 2: make_public ile (public fields)
#[make_public]
#[derive(Debug)]
pub struct PublicPrompt {
    message: String,
    level: String,
    word_count: usize,
    time_remaining: i32,
}
