use ibus_vie_im::{Engine, VniEngine};

fn feed(input: &str) -> String {
    let mut engine = VniEngine::new();
    engine.feed_str(input)
}

#[test]
fn basic() {
    assert_eq!(feed("a"), "a");
    assert_eq!(feed("ba"), "ba");
}

#[test]
fn tone() {
    assert_eq!(feed("a1"), "á");
    assert_eq!(feed("a2"), "à");
}

#[test]
fn circumflex() {
    assert_eq!(feed("a6"), "â");
    assert_eq!(feed("e6"), "ê");
}

#[test]
fn horn() {
    assert_eq!(feed("o7"), "ơ");
    assert_eq!(feed("u7"), "ư");
}

#[test]
fn breve() {
    assert_eq!(feed("a8"), "ă");
}

#[test]
fn d_stroke() {
    assert_eq!(feed("d9"), "đ");
}

#[test]
fn viet() {
    assert_eq!(feed("vie6t5"), "việt");
}

#[test]
fn dau() {
    assert_eq!(feed("d9a6u"), "đâu");
}
