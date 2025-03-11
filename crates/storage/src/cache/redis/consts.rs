const MALLOC_SEQ: &str = "MALLOC_SEQ:";
const MESSAGE_CACHE: &str = "MSG_CACHE:";

pub fn get_malloc_seq_key(conversation_id: &str) -> String {
    format!("{}{}", MALLOC_SEQ, conversation_id)
}

pub fn get_msg_cache_key(conversation_id: &str, seq: i64) -> String {
    format!("{}{}:{}", MESSAGE_CACHE, conversation_id, seq)
}
