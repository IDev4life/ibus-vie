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
