//! Checks on the Internals wiki (`docs/internals/`, `add-internals-wiki`):
//! the pages GitHub's wiki publishes from the repository.
//!
//! - Every page but `Home`, `Glossary`, `_Sidebar` and `_Footer` ends with a
//!   `## Source files` section of repository links.
//! - Every repository link names a file or directory that exists.
//! - Every non-test Rust file under `src/`, and every top-level item of
//!   `web/`, `scripts/`, `tests/`, `benches/` and `.github/workflows/`, is
//!   named in some page's "Source files" section.
//! - Every link between pages, and to a heading on one, resolves.
//! - The sidebar links every page.
//!
//! Each check lists every offending page and item at once. Pages can still
//! go stale in what they say; the rule in `CLAUDE.md` covers that.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const REPO_URL: &str = "https://github.com/GerMP21/Corrosion-VST/";
const SOURCE_FILES: &str = "Source files";
/// Pages with no "Source files" section.
const EXEMPT: [&str; 4] = ["Home", "Glossary", "_Sidebar", "_Footer"];
/// Directories whose top-level items must each be named by a page.
const TOP_LEVEL_ROOTS: [&str; 5] = ["web", "scripts", "tests", "benches", ".github/workflows"];
/// Items under those roots that need no page: lockfiles, and build outputs
/// and caches that aren't in the repository.
const IGNORED_ITEMS: [&str; 7] = [
    "Cargo.lock",
    "package-lock.json",
    "dist",
    "node_modules",
    "target",
    "vendor",
    "__pycache__",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn wiki_dir() -> PathBuf {
    root().join("docs/internals")
}

/// The wiki's pages: page name (file stem) → text, with `\r\n` normalized.
fn pages() -> BTreeMap<String, String> {
    let mut pages = BTreeMap::new();
    for entry in fs::read_dir(wiki_dir()).expect("docs/internals/ exists") {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "md") {
            let name = path.file_stem().unwrap().to_string_lossy().into_owned();
            let text = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
            pages.insert(name, text);
        }
    }
    pages
}

/// The page's text with fenced code blocks and inline code spans blanked,
/// so links inside code don't count.
fn strip_code(text: &str) -> String {
    strip_code_with(text, false)
}

/// The page's text with fenced code blocks blanked and, if `keep_inline`,
/// inline code kept as its text without the backticks (as GitHub does when
/// it builds a heading's anchor).
fn strip_code_with(text: &str, keep_inline: bool) -> String {
    let mut out = String::new();
    let mut fence: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(marker) = &fence {
            if trimmed.starts_with(marker.as_str()) {
                fence = None;
            }
            out.push('\n');
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let marker: String = trimmed
                .chars()
                .take_while(|&c| c == '`' || c == '~')
                .collect();
            fence = Some(marker);
            out.push('\n');
            continue;
        }
        // Inline code: drop everything between matching backtick runs.
        let mut in_code = false;
        for c in line.chars() {
            if c == '`' {
                in_code = !in_code;
            } else if !in_code || keep_inline {
                out.push(c);
            }
        }
        out.push('\n');
    }
    out
}

/// Every Markdown link target `[text](target)` in `text` (code excluded).
fn link_targets(text: &str) -> Vec<String> {
    let text = strip_code(text);
    let mut targets = Vec::new();
    let mut rest = text.as_str();
    while let Some(i) = rest.find("](") {
        let after = &rest[i + 2..];
        if let Some(end) = after.find(')') {
            let target = after[..end].trim();
            // `[text](<url> "title")` and `[text](url "title")`
            let target = target.split_whitespace().next().unwrap_or("");
            let target = target.trim_start_matches('<').trim_end_matches('>');
            targets.push(target.to_string());
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    targets
}

/// The body of the `## <name>` section: from its heading to the next `## `
/// heading or the end of the page. `None` when the page has no such section.
fn section<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let heading = format!("## {name}");
    let mut offset = 0;
    let mut start = None;
    for line in text.split_inclusive('\n') {
        let bare = line.trim_end();
        if let Some(s) = start {
            if bare.starts_with("## ") {
                return Some(&text[s..offset]);
            }
        } else if bare == heading {
            start = Some(offset + line.len());
        }
        offset += line.len();
    }
    start.map(|s| &text[s..])
}

/// GitHub's anchor for a heading's text: links reduced to their text, lower
/// case, everything but letters, digits, spaces, `-` and `_` removed, spaces
/// turned into `-`.
fn slug(heading: &str) -> String {
    // [text](url) → text
    let mut plain = String::new();
    let mut rest = heading;
    while let Some(open) = rest.find('[') {
        plain.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match (after.find("]("), after.find(')')) {
            (Some(close), Some(end)) if close < end => {
                plain.push_str(&after[..close]);
                rest = &after[end + 1..];
            }
            _ => {
                plain.push('[');
                rest = after;
            }
        }
    }
    plain.push_str(rest);
    plain
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// Every anchor a page's headings produce, with GitHub's `-1`, `-2` suffixes
/// on repeats.
fn anchors(text: &str) -> BTreeSet<String> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut anchors = BTreeSet::new();
    for line in strip_code_with(text, true).lines() {
        let hashes = line.chars().take_while(|&c| c == '#').count();
        if (1..=6).contains(&hashes) && line[hashes..].starts_with(' ') {
            let base = slug(&line[hashes + 1..]);
            let count = seen.entry(base.clone()).or_insert(0);
            let anchor = if *count == 0 {
                base.clone()
            } else {
                format!("{base}-{count}")
            };
            *count += 1;
            anchors.insert(anchor);
        }
    }
    anchors
}

/// A repository link's kind and path: `("blob", "src/lib.rs")`, without any
/// `#fragment` or `?query`. `None` for anything else.
fn repository_path(target: &str) -> Option<(&str, &str)> {
    let rest = target.strip_prefix(REPO_URL)?;
    let (kind, path) = if let Some(path) = rest.strip_prefix("blob/main/") {
        ("blob", path)
    } else {
        ("tree", rest.strip_prefix("tree/main/")?)
    };
    let path = path.split(['#', '?']).next().unwrap();
    Some((kind, path.trim_end_matches('/')))
}

/// Paths named in a page's "Source files" section.
fn named_paths(text: &str) -> Vec<String> {
    section(text, SOURCE_FILES)
        .map(|body| {
            link_targets(body)
                .iter()
                .filter_map(|t| repository_path(t).map(|(_, p)| p.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn is_test_file(relative: &str) -> bool {
    let name = relative.rsplit('/').next().unwrap();
    name == "tests.rs"
        || name.ends_with("_tests.rs")
        || relative.split('/').rev().skip(1).any(|dir| dir == "tests")
}

/// Every `.rs` file under `dir`, as a repository-relative path with `/`.
fn rust_files(dir: &Path, out: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let relative = path.strip_prefix(root()).unwrap();
            out.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

#[test]
fn every_page_lists_its_source_files() {
    let mut problems = Vec::new();
    for (name, text) in pages() {
        if EXEMPT.contains(&name.as_str()) {
            continue;
        }
        let Some(body) = section(&text, SOURCE_FILES) else {
            problems.push(format!("{name}: no `## {SOURCE_FILES}` section"));
            continue;
        };
        let targets = link_targets(body);
        if targets.is_empty() {
            problems.push(format!("{name}: `## {SOURCE_FILES}` lists no link"));
        }
        for target in targets {
            if repository_path(&target).is_none() {
                problems.push(format!(
                    "{name}: `{target}` in `## {SOURCE_FILES}` isn't a {REPO_URL}blob/main/ or tree/main/ link"
                ));
            }
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}

#[test]
fn every_linked_repository_path_exists() {
    let mut problems = Vec::new();
    for (name, text) in pages() {
        for target in link_targets(&text) {
            let Some((kind, path)) = repository_path(&target) else {
                continue;
            };
            let full = root().join(path);
            let ok = match kind {
                "blob" => full.is_file(),
                _ => full.is_dir(),
            };
            if !ok {
                let what = if kind == "blob" { "file" } else { "directory" };
                problems.push(format!("{name}: no {what} `{path}`"));
            }
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}

#[test]
#[ignore = "pages being written (add-internals-wiki)"]
fn every_source_file_is_named() {
    let named: BTreeSet<String> = pages().values().flat_map(|t| named_paths(t)).collect();
    let mut problems = Vec::new();

    let mut sources = Vec::new();
    rust_files(&root().join("src"), &mut sources);
    for file in sources {
        if !is_test_file(&file) && !named.contains(&file) {
            problems.push(format!("{file}: no page names it"));
        }
    }

    for top in TOP_LEVEL_ROOTS {
        for entry in fs::read_dir(root().join(top)).unwrap() {
            let item = entry.unwrap().file_name().to_string_lossy().into_owned();
            if item.starts_with('.') || IGNORED_ITEMS.contains(&item.as_str()) {
                continue;
            }
            let path = format!("{top}/{item}");
            let prefix = format!("{path}/");
            if !named.iter().any(|n| *n == path || n.starts_with(&prefix)) {
                problems.push(format!("{path}: no page names it or anything under it"));
            }
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}

#[test]
#[ignore = "pages being written (add-internals-wiki): links to pages not yet written"]
fn wiki_links_resolve() {
    let pages = pages();
    let anchors: BTreeMap<&str, BTreeSet<String>> = pages
        .iter()
        .map(|(name, text)| (name.as_str(), anchors(text)))
        .collect();
    let mut problems = Vec::new();
    for (name, text) in &pages {
        for target in link_targets(text) {
            if target.is_empty()
                || target.contains("://")
                || target.starts_with('/')
                || target.starts_with("mailto:")
            {
                continue;
            }
            let (page, fragment) = match target.split_once('#') {
                Some((page, fragment)) => (page, Some(fragment)),
                None => (target.as_str(), None),
            };
            let page = if page.is_empty() { name.as_str() } else { page };
            let Some(page_anchors) = anchors.get(page) else {
                problems.push(format!("{name}: `{target}` names no page"));
                continue;
            };
            if let Some(fragment) = fragment {
                if !page_anchors.contains(fragment) {
                    problems.push(format!(
                        "{name}: `{target}`: {page} has no heading `#{fragment}`"
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}

#[test]
#[ignore = "pages being written (add-internals-wiki)"]
fn sidebar_links_every_page() {
    let pages = pages();
    let sidebar = pages
        .get("_Sidebar")
        .expect("docs/internals/_Sidebar.md exists");
    let linked: BTreeSet<String> = link_targets(sidebar)
        .into_iter()
        .map(|t| t.split('#').next().unwrap().to_string())
        .collect();
    let missing: Vec<&String> = pages
        .keys()
        .filter(|name| !name.starts_with('_') && !linked.contains(*name))
        .collect();
    assert!(missing.is_empty(), "_Sidebar doesn't link: {missing:?}");
}

// Helper tests, on fixed strings.

#[test]
fn slugs_follow_github() {
    assert_eq!(slug("Source files"), "source-files");
    assert_eq!(slug("The `Voice` loop: why?"), "the-voice-loop-why");
    assert_eq!(
        slug("Drive levels & the governor"),
        "drive-levels--the-governor"
    );
    assert_eq!(slug("[Link](Page) text"), "link-text");
    assert_eq!(slug("Rust, Damage and Heat"), "rust-damage-and-heat");
    let page = "# A\n## Notes\n### Notes\n```\n## Not a heading\n```\n## Sizes: `MAX_MODES`\n";
    let found = anchors(page);
    assert_eq!(
        found,
        ["a", "notes", "notes-1", "sizes-max_modes"]
            .map(String::from)
            .into_iter()
            .collect()
    );
}

#[test]
fn links_skip_code() {
    let text =
        "See [a](Page-A) and `[b](Page-B)`.\n```\n[c](Page-C)\n```\n[d](Page-D#x \"title\")\n";
    assert_eq!(link_targets(text), ["Page-A", "Page-D#x"]);
}

#[test]
fn sections_end_at_the_next_level_two_heading() {
    let text = "# T\n## Source files\n- [x](u)\n### Sub\n- [y](v)\n## After\n- [z](w)\n";
    let body = section(text, SOURCE_FILES).unwrap();
    assert_eq!(link_targets(body), ["u", "v"]);
    assert!(section(text, "Missing").is_none());
}

#[test]
fn repository_paths_are_parsed() {
    assert_eq!(
        repository_path(&format!("{REPO_URL}blob/main/src/lib.rs#L10")),
        Some(("blob", "src/lib.rs"))
    );
    assert_eq!(
        repository_path(&format!("{REPO_URL}tree/main/src/dsp/")),
        Some(("tree", "src/dsp"))
    );
    assert_eq!(repository_path("https://example.com/x"), None);
    assert_eq!(
        repository_path(&format!("{REPO_URL}blob/dev/src/lib.rs")),
        None
    );
    assert!(is_test_file("src/dsp/tests.rs"));
    assert!(is_test_file("src/dsp/exciters/event_timing_tests.rs"));
    assert!(is_test_file("src/x/tests/helpers.rs"));
    assert!(!is_test_file("src/dsp/interaction.rs"));
}
