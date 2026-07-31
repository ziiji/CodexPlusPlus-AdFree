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
    let actual = format!("{:X}", Sha256::digest(bytes));
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
            "74D3BFB0F0F55C138EE3B0933F7B55BC11F71D09F1B07794BCA51B6598DE203D",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/dream-skin.css",
            "12848DA7DDAACF1B0F18CD419B2B27A6355DCBCE01AF650F1BCE14D99FEBD532",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/renderer-inject.js",
            "2704C39506C66554C3529BF0D15B876B4AEC2DD9A36B1796AB43E19C33A046FC",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/dream-skin.css",
            "EC3C3BC5F6E10E20A3F2307796BD1E1350E80E5D23D37318EE5468833C95A6DF",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/renderer-inject.js",
            "0BFB5F66A0323BF1392B42033E66904DE3EC4BFC8A5BA297F2BB92A4A6740A34",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/dream-skin.css",
            "482A60AF98DD6B460BF624C56918C5B57F9CCD5B55E52FA46D486F7D65259D9A",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/renderer-inject.js",
            "21FAF1DC0A3EBE78D8D972182CACE62BD93D5D0E5841725398A4A524EF2BC20B",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/dream-skin.css",
            "5E149E9A13985961C5F3125296178ACB2ABF0B528974F1E616AA625970430562",
        ),
        (
            "assets/inject/upstream/snow-skin/renderer-inject.js",
            "0FCDFF4AECD03EAB2CA4EE923CCD20CB97EB5460F7C9F07351A2003FFA76E6FA",
        ),
        (
            "assets/inject/upstream/snow-skin/dream-skin.css",
            "0AF2D20FBE3E3DD13F0BE7F1E5A90366E1501084827B22C1D4815A421BFCE823",
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
