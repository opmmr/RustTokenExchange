use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static TOKEN_CACHE: Lazy<Mutex<HashMap<u32, Token>>> = Lazy::new(|| Mutex::new(HashMap::new()));
#[derive(Clone, Debug)]
struct Token {
    id: u32,
}

static PENDING_FETCHES: Lazy<Mutex<HashSet<u32>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn fetch_and_cache_tokens() {
    let token_ids: Vec<u32>;
    {
        let mut pending = PENDING_FETCHES.lock().unwrap();
        if pending.is_empty() {
            return;
        }
        token_ids = pending.drain().collect();
    }

    let fetched_tokens: Vec<Token> = fetch_tokens_in_batch(token_ids);
    
    let mut cache = TOKEN_CACHE.lock().unwrap();
    for token in fetched_keys {
        cache.insert(token.id, token.clone());
    }
}

fn fetch_tokens_in_batch(token_ids: Vec<u32>) -> Vec<Token> {
    vec![]
}

fn get_token_details(token_id: u32) -> Option<Token> {
    {
        let cache = TOKEN_CACHE.lock().unwrap();
        if let Some(token) = cache.get(&token_id) {
            return Some(token.clone());
        }
    }

    {
        let mut pending = PENDING_FETCHES.lock().unwrap();
        pending.insert(token_id);
    }

    fetch_and_cache_tokens();

    let cache = TOKEN_CACHE.lock().unwrap();
    cache.get(&token_id).cloned()
}

fn main() {
}