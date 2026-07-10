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
            "97C1F062F6695C19469E851390974121F29C66B690C4790E761C0E1F82586EF1",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/dream-skin.css",
            "3CEF5BD3D536EDA5F1802B325B9727668B6701BE51C9BA613AB7734D203BFCD9",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/renderer-inject.js",
            "806D23E953CE356DA621E164467141E7CA8B28235562F4252386FDABF952A5B5",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/dream-skin.css",
            "BBC44EBE8EEAA6A8F25BB00559C30294816FB185A86CD2BD425E061BFF57E05F",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/renderer-inject.js",
            "97C1F062F6695C19469E851390974121F29C66B690C4790E761C0E1F82586EF1",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/dream-skin.css",
            "82ECECF50F3595B80BD148D55246FA2871E3F3D0A2C9031F5BAD8B5E6413E666",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/renderer-inject.js",
            "09F5BF89BFD8DA90E0E3FD74CA840AD417B63BDF6E5DFDBCB720FD1A6B1FF54E",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/dream-skin.css",
            "662B04F2E74570770394E3D6F012F6B5952C50E55A1AD47B1577161F860D775D",
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
            "57A529C0F5743CC7068B5F9064AAB098137520A051E5B0C5A45AD2DFAB91E98C",
        ),
        (
            "assets/inject/upstream/glass-vision/glass-vision.css",
            "84D4AF19D9D5B7D5934139892F83CDB58B5EB370598D775A54587C285A2C8BC1",
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
