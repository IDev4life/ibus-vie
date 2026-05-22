use ibus_vie_im::{Engine, TelexEngine};

fn feed(input: &str) -> String {
    let mut engine = TelexEngine::new();
    engine.feed_str(input)
}

#[test]
fn basic_vowel() {
    assert_eq!(feed("a"), "a");
    assert_eq!(feed("ba"), "ba");
}

#[test]
fn tone_acute() {
    assert_eq!(feed("as"), "á");
}

#[test]
fn viet() {
    assert_eq!(feed("vieetj"), "việt");
}

#[test]
fn tieng() {
    assert_eq!(feed("tieengs"), "tiếng");
}

#[test]
fn dd() {
    assert_eq!(feed("ddi"), "đi");
}

#[test]
fn chao() {
    assert_eq!(feed("chaof"), "chào");
}

#[test]
fn nguoi() {
    assert_eq!(feed("nguwowif"), "người");
}

#[test]
fn space_commit() {
    let mut engine = TelexEngine::new();
    let result = engine.feed_str("xin chao");
    assert_eq!(result, "xin chao");
}

#[test]
fn ddau() {
    assert_eq!(feed("ddaau"), "đâu");
}

#[test]
fn uppercase() {
    assert_eq!(feed("DDi"), "Đi");
}

#[test]
fn w_ua_pair() {
    assert_eq!(feed("chuaw"), "chưa");
}

#[test]
fn w_uo_pair() {
    assert_eq!(feed("dduocwj"), "được");
}

#[test]
fn w_uo_thuong() {
    assert_eq!(feed("thuowng"), "thương");
}

#[test]
fn w_single_u() {
    assert_eq!(feed("tuw"), "tư");
}

#[test]
fn w_single_a() {
    assert_eq!(feed("aw"), "ă");
}

#[test]
fn flexible_mark_oo_after_coda() {
    // 'o' after coda "ng" marks earlier 'o' to 'ô'
    assert_eq!(feed("khongo"), "không");
}

#[test]
fn flexible_mark_ee_after_vowel() {
    // 'e' after 'u' marks earlier 'e' to 'ê', then tone
    assert_eq!(feed("nhieuef"), "nhiều");
}

#[test]
fn tone_double_press_undo() {
    // Second 'r' undoes hook tone and appends literal 'r'
    assert_eq!(feed("xayrr"), "xayr");
}

#[test]
fn tone_double_press_undo_s() {
    // Second 's' undoes acute tone and appends literal 's'
    assert_eq!(feed("bass"), "bas");
}
