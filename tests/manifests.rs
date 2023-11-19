use synthesizer::{manifests, utils};

#[test]
fn check_default_manifest() {
    let filepath = "./data/synth_manifest.yml";
    let raw_manifest = utils::load_file(filepath);
    let manifest = manifests::parse_manifest_file(raw_manifest);
    assert!(!manifest.pipelines.is_empty());
    assert!(!manifest.tasks.is_empty());
}
