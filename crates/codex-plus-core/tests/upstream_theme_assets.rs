use sha2::{Digest, Sha256};

fn assert_sha256(relative_path: &str, expected: &str) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative_path);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "failed to read upstream theme asset {}: {error}",
            path.display()
        )
    });
    // Git may materialize these text assets with CRLF on Windows. Hash the
    // repository's canonical LF representation so the guard is platform
    // independent while still detecting substantive asset changes.
    let mut normalized = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
            index += 1;
        }
        normalized.push(bytes[index]);
        index += 1;
    }
    let actual = format!("{:X}", Sha256::digest(normalized));
    assert_eq!(actual, expected, "upstream asset changed: {relative_path}");
}

fn assert_text_sha256(relative_path: &str, expected: &str) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative_path);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "failed to read upstream theme asset {}: {error}",
            path.display()
        )
    });
    let normalized = text.replace("\r\n", "\n");
    let actual = format!("{:X}", Sha256::digest(normalized.as_bytes()));
    assert_eq!(actual, expected, "upstream asset changed: {relative_path}");
}

fn assert_no_promotional_fields(relative_path: &str) {
    fn visit(value: &serde_json::Value, relative_path: &str) {
        match value {
            serde_json::Value::Object(fields) => {
                for key in ["promoTitle", "promoSub", "promoUrl"] {
                    assert!(
                        !fields.contains_key(key),
                        "promotional field {key} found in {relative_path}"
                    );
                }
                for value in fields.values() {
                    visit(value, relative_path);
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    visit(value, relative_path);
                }
            }
            _ => {}
        }
    }

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative_path);
    let value: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&path)
            .unwrap_or_else(|error| panic!("failed to read theme {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("failed to parse theme {}: {error}", path.display()));
    visit(&value, relative_path);
}

#[test]
fn bundled_target_renderers_and_styles_remain_byte_exact() {
    for (path, hash) in [
        (
            "assets/inject/upstream/dream-skin/windows/renderer-inject.js",
            "18E5E99719042A3D877915F70C0D023B6914C8458EF004B40F5FF8E9FE6D5C3F",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/dream-skin.css",
            "049695F3F8FD66826F7DD0EF9363D21A5AA491C627DD9602CEAEA7383CFDD49C",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/renderer-inject.js",
            "73DA118C964E768676C44C9ABAC910114547DDA44B6190CC3D8A6220059ABB0B",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/dream-skin.css",
            "CDA12A5E08815533919A6005A803C2269637CDCAAC4D121D170230163DC9CF09",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/renderer-inject.js",
            "98B9F323A9711FFB3E8307C9ACA3F4FBDA71896A047F334AB38AF43FDB2D53E6",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/dream-skin.css",
            "0C371B7D794C4783648D1733661E8FA8674C872296CE5CF9898B28EB1765425C",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/renderer-inject.js",
            "19202C8A37C7512E65F950A5516A314867FDF305B74B313F0ABCEA8CF7347F59",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/dream-skin.css",
            "45506CA7C71D4E9867287AE2358C4380C0993F0D04039C29FEE6DBEE20495148",
        ),
        (
            "assets/inject/upstream/snow-skin/renderer-inject.js",
            "9AE8123B51917975B5D4B91995173A6A4DD3C27C6BD5B465B5670C2C1330955A",
        ),
        (
            "assets/inject/upstream/snow-skin/dream-skin.css",
            "97807DE20E40680471D211466B657867CB46280F393EF9D7FBBA5CE829AE5599",
        ),
        (
            "assets/inject/upstream/glass-vision/renderer-inject.js",
            "D14943E95DB62DB81BF29D9CF14FCAF1DD1EA9A9625245C020865127EEA295A2",
        ),
        (
            "assets/inject/upstream/glass-vision/glass-vision.css",
            "4C37C53544EE4F1CD93BA5D0DC3E174B05D4CB84EC9A436295D11D19F0BB04F1",
        ),
    ] {
        assert_sha256(path, hash);
    }
}

#[test]
fn bundled_ad_free_theme_files_remain_content_exact() {
    for (path, hash) in [
        (
            "assets/inject/upstream/dream-skin/macos/theme.json",
            "50FCD415B210BEC9FDA9CA6DED3660B3730877C94B9C72A14B7B3E452B5CB229",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/theme.json",
            "50FCD415B210BEC9FDA9CA6DED3660B3730877C94B9C72A14B7B3E452B5CB229",
        ),
        (
            "assets/inject/upstream/skin-packs/packs/caishen-lite/theme.json",
            "CB58204AC17B5D73859193A6C7C1EE4CE33568DEE88582D5B9AC2CEB35E0B6D7",
        ),
        (
            "assets/inject/upstream/skin-packs/packs/caishen-max/theme.json",
            "AF1E5844A8015EC6B8AF7ABCF28D002A1A3A5AF99F95F1D492A42175F096EF84",
        ),
        (
            "assets/inject/upstream/skin-packs/packs/caishen-readable/theme.json",
            "9EA843898488827C4A311644F69E2BBE8DF3C98F09E24C942EA2B68D8E98D299",
        ),
        (
            "assets/inject/upstream/skin-packs/packs/export-night/theme.json",
            "836738F5EF3C10142463671050D898E90F5B25C7E391FA7A4E0280C7FA391312",
        ),
        (
            "assets/inject/upstream/skin-packs/packs/global-founder-bright/theme.json",
            "0816234B0BC8AE64194B7327168F9C353998FC17E4A5178B8E9BF1984B1820BB",
        ),
        (
            "assets/inject/upstream/skin-packs/packs/mythic-guardian-noir/theme.json",
            "0D984F68A0B6841E72CE062E6E56830B4FA305E6B8A1808A97A2B21E9F6F0B38",
        ),
    ] {
        assert_text_sha256(path, hash);
        assert_no_promotional_fields(path);
    }
}
