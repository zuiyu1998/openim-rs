pub fn gen_conversation_id_for_single(send_id: &str, recv_id: &str) -> String {
    let mut sorts = vec![send_id, recv_id];

    sorts.sort();
    return format!("si_{}", sorts.join("_"));
}

pub fn gen_conversation_unique_key_for_single(send_id: &str, recv_id: &str) -> String {
    let mut sorts = vec![send_id, recv_id];

    sorts.sort();
    return format!("{}", sorts.join("_"));
}
