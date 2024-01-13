use poise::serenity_prelude as serenity;

pub fn check_for_dn(message: &serenity::Message) -> bool {
    let keyword = "dn";
    let msg = clean_dn(&message.content);
    msg.split(' ').count() <= 6 && msg.contains(keyword)
}

/// removes common prefixes and suffixes from dn, lowercases dn
fn clean_dn(message: &str) -> String {
    message
        .to_lowercase()
        .replace("hbu", "")
        .replace("how about you", "")
}
