use crate::config::{Codecov, CustomBadge, Package};

pub fn create(package: &Package) -> String {
    let mut badges = Vec::new();

    let Package {
        name,
        version,
        license,
        metadata,
        repository,
        ..
    } = &package;

    let badge_style = &metadata.badge_style;

    if metadata.badges.docs_rs {
        badges.push(format!("[![docs.rs](https://img.shields.io/docsrs/{name}/{version}.svg?logo=docs.rs&label=docs.rs&style={badge_style})](https://docs.rs/{name}/{version})"));
    }

    if metadata.badges.crates_io.release() {
        badges.push(format!("[![crates.io](https://img.shields.io/badge/crates.io-v{version}-orange?style={badge_style}&logo=rust&logoColor=white)](https://crates.io/crates/{name}/{version})"));
    }

    if metadata.badges.license
        && let Some(license) = license
    {
        badges.push(format!(
            "![License: {license}](https://img.shields.io/badge/license-{escaped_license}-purple.svg?style={badge_style})",
            escaped_license = license.replace(' ', "%20").replace('-', "--"),
        ));
    }

    if metadata.badges.crates_io.size() {
        badges.push(format!(
            "![Crates.io Size](https://img.shields.io/crates/size/{name}/{version}.svg?style={badge_style})"
        ));
    }

    if metadata.badges.crates_io.downloads() {
        badges.push(format!("![Crates.io Downloads](https://img.shields.io/crates/dv/{name}/{version}.svg?&label=downloads&style={badge_style})"));
    }

    let repository = repository
        .as_ref()
        .and_then(|r| r.strip_prefix("https://github.com/"));

    match (&metadata.badges.codecov, repository) {
        (Codecov::Simple(false), _) => {}
        (Codecov::Simple(true), Some(repository)) => {
            badges.push(format!("[![Codecov](https://img.shields.io/codecov/c/github/{repository}.svg?label=codecov&logo=codecov&style={badge_style})](https://app.codecov.io/gh/{repository})"))
        }
        (Codecov::Complex { component }, Some(repository)) => {
            badges.push(format!("[![Codecov](https://img.shields.io/codecov/c/github/{repository}.svg?label=codecov&logo=codecov&style={badge_style}&component={component})](https://app.codecov.io/gh/{repository})"))
        },
        _ => {
            eprintln!("codecov badge currently only supported for github repositories");
        }
    }

    for CustomBadge {
        link,
        name: text,
        url,
    } in &metadata.custom_badges
    {
        let badge = format!("![{text}]({url})");
        if let Some(link) = link {
            badges.push(format!("[{badge}]({link})"))
        } else {
            badges.push(badge);
        }
    }

    badges.join("\n")
}
