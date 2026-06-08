// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure launchd output and plist parsers used by the service adapter.

use std::path::PathBuf;

fn xml_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

pub(super) fn parse_launchd_last_exit_status(stdout: &str) -> Option<i32> {
    for line in stdout.lines() {
        let trimmed = line.trim();
        let Some(value) = trimmed.strip_prefix("\"LastExitStatus\" = ") else {
            continue;
        };
        if let Ok(exit_code) = value.trim_end_matches(';').trim().parse::<i32>() {
            return Some(exit_code);
        }
    }

    None
}

pub(crate) fn parse_launchd_disabled_services(output: &str, label: &str) -> Option<bool> {
    for line in output.lines() {
        let normalized = line.trim().trim_end_matches(';').trim_end_matches(',');
        let Some((raw_key, raw_value)) = normalized
            .split_once("=>")
            .or_else(|| normalized.split_once('='))
        else {
            continue;
        };

        let key = raw_key.trim().trim_matches('"');
        if key != label {
            continue;
        }

        let value = raw_value
            .trim()
            .trim_matches('"')
            .trim_end_matches(';')
            .trim_end_matches(',')
            .to_ascii_lowercase();
        return match value.as_str() {
            "true" | "1" => Some(false),
            "false" | "0" => Some(true),
            _ => None,
        };
    }

    None
}

pub(crate) fn parse_launchd_log_path(plist_content: &str) -> Option<PathBuf> {
    let mut expect_path_value = false;

    for line in plist_content.lines() {
        let trimmed = line.trim();
        if expect_path_value {
            let value = trimmed
                .strip_prefix("<string>")?
                .strip_suffix("</string>")?;
            return Some(PathBuf::from(xml_unescape(value)));
        }

        if trimmed == "<key>StandardOutPath</key>" || trimmed == "<key>StandardErrorPath</key>" {
            expect_path_value = true;
        }
    }

    None
}

pub(crate) fn parse_launchd_data_dir(plist_content: &str) -> Option<PathBuf> {
    for line in plist_content.lines() {
        let trimmed = line.trim();
        let Some(value) = trimmed
            .strip_prefix("<string>-datadir=")
            .and_then(|value| value.strip_suffix("</string>"))
        else {
            continue;
        };
        return Some(PathBuf::from(xml_unescape(value)));
    }

    None
}
