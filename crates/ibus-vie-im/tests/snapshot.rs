//! Snapshot test runner — reads tests/snapshot/*.txt from workspace root
//! and verifies each input→expected pair via engine.feed_str().

use ibus_vie_im::{Engine, TelexEngine, VniEngine};
use std::path::Path;

fn snapshot_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/snapshot"))
}

fn run_snapshot(path: &Path, mut engine_factory: impl FnMut() -> Box<dyn Engine>) {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));

    for (line_num, line) in content.lines().enumerate() {
        let line_num = line_num + 1;
        // Skip comments and empty lines
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(
            parts.len(),
            2,
            "{}:{}: expected TAB-separated input<TAB>expected, got: {:?}",
            path.display(),
            line_num,
            line
        );

        let input = parts[0];
        let expected = parts[1];
        let mut engine = engine_factory();
        let actual = engine.feed_str(input);

        assert_eq!(
            actual, expected,
            "{}:{}: input={:?} expected={:?} got={:?}",
            path.display(),
            line_num,
            input,
            expected,
            actual
        );
    }
}

#[test]
fn snapshot_telex() {
    let path = snapshot_dir().join("telex.txt");
    run_snapshot(&path, || Box::new(TelexEngine::new()));
}

#[test]
fn snapshot_vni() {
    let path = snapshot_dir().join("vni.txt");
    run_snapshot(&path, || Box::new(VniEngine::new()));
}
