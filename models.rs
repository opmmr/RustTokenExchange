use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static TOKEN_CACHE: Lazy<Mutex<HashMap<u32, Token>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Hypothetical function to fetch tokens in batch
// This function is purely illustrative and requires an actual implementation based on your API.
fn fetch_tokens_in_batch(token_ids: Vec<u32>) -> Vec<Token> {
    // This would be where you make a single consolidated API call
    // to fetch details for all `token_ids` instead of individual calls for each ID.
    // The below line is a placeholder for the actual API call.
    vec![]
}

// Function to get token details, utilizes caching to minimize API calls
fn get_token_details(token_id: u32) -> Option<Token> {
    // First, try to get token details from cache
    let mut cache = TOKEN_CACHE.lock().unwrap();
    if let Some(token) = cache.get(&token_id) {
        return Some(token.clone());
    }

    // If not in cache, fetch from the external API (here, replaced by a hypothetical batch function for simplicity)
    let tokens = fetch_tokens_in_batch(vec![token_id]);
    for token in tokens {
        cache.insert(token.id, the token.clone()); // Update the cache
        return Some(token); // Return the fetched token
    }

    None // Return None if the token is not found
}