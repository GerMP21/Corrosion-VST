#!/usr/bin/env python3
"""Regenerate the Rust crate section of THIRD-PARTY-NOTICES.md.

Corrosion's distributed builds (the plugin bundles for Linux, Windows and
macOS, and the web app's wasm) link third-party crates whose licenses ask for
their notices to travel with the binaries. This script lists every crate those
builds depend on and writes their license texts between the GENERATED markers
of THIRD-PARTY-NOTICES.md. The hand-written sections around the markers
(repository material, the site's npm packages, trademarks) are left alone.

Which crates: the normal (non-dev, non-build) dependency closure of the
`corrosion` package for each target the bundles are built for, plus that of
`corrosion-web-engine` and `corrosion-web-editor` for wasm32. Corrosion's own
packages are left out.

Which texts: the crate's own license files (LICENSE*, COPYING*, NOTICE*, ...,
plus a few named in EXTRA_FILES). A crate published without one gets the
license files of its git repository's root when that root declares the same
license, and otherwise the standard text of a license its SPDX expression
offers, with the authors from its Cargo.toml as the copyright holders.

Usage (from anywhere in the repository; needs network or a warm Cargo cache):
    python3 scripts/third_party_notices.py          # rewrite the file
    python3 scripts/third_party_notices.py --check  # fail if it is stale

It runs `web/scripts/fetch-nih-plug.sh` first, because the web workspace
patches NIH-plug from web/vendor/nih-plug. A license that isn't on the
GPLv3-compatible allowlist (ALLOWED) fails the run, naming the crate, so a new
license gets a human look before it ships.
"""

import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NOTICES = os.path.join(ROOT, "THIRD-PARTY-NOTICES.md")
BEGIN = "<!-- BEGIN GENERATED: scripts/third_party_notices.py -->"
END = "<!-- END GENERATED -->"

# (workspace directory, root packages, targets): what the bundles and the web
# app are built from (scripts/bundle.sh, web/scripts/build.sh). Both Windows
# ABIs: CI ships MSVC, the maintainer builds GNU.
WORKSPACES = [
    (
        ".",
        {"corrosion"},
        [
            "x86_64-unknown-linux-gnu",
            "x86_64-pc-windows-msvc",
            "x86_64-pc-windows-gnu",
            "x86_64-apple-darwin",
            "aarch64-apple-darwin",
        ],
    ),
    ("web", {"corrosion-web-engine", "corrosion-web-editor"}, ["wasm32-unknown-unknown"]),
]
OURS = {"corrosion", "corrosion-web-engine", "corrosion-web-editor"}

LICENSE_FILE = re.compile(
    r"^(licen[cs]e|copying|notice|unlicense|copyright)([-._].*)?$", re.IGNORECASE
)
# License files a crate keeps under another name or in a subdirectory.
EXTRA_FILES = {}

# Crates that must not ship, and why. Finding one fails the run.
FORBIDDEN = {
    "epaint_default_fonts": (
        "egui's default fonts (Ubuntu Font Licence, OFL emoji); the editor embeds "
        "its own (src/gui/fonts.rs). Something turned egui's `default_fonts` back on: "
        "see vendor/egui-baseview/PATCHED.md"
    ),
}

# GPLv3-compatible licenses. Anything else fails the run.
ALLOWED = {
    "0BSD", "Apache-2.0", "BlueOak-1.0.0", "BSD-1-Clause", "BSD-2-Clause",
    "BSD-3-Clause", "BSL-1.0", "CC0-1.0", "GPL-3.0", "GPL-3.0-only",
    "GPL-3.0-or-later", "ISC", "LLVM-exception", "MIT", "MIT-0", "MPL-2.0",
    "Unicode-3.0", "Unicode-DFS-2016", "Unlicense", "Zlib",
}
ALIASES = {"GPLv3": "GPL-3.0"}

MIT_TEMPLATE = """MIT License

Copyright (c) {holders}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."""

ISC_TEMPLATE = """ISC License

Copyright (c) {holders}

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE."""

BSL_TEXT = """Boost Software License - Version 1.0 - August 17th, 2003

Permission is hereby granted, free of charge, to any person or organization
obtaining a copy of the software and accompanying documentation covered by
this license (the "Software") to use, reproduce, display, distribute,
execute, and transmit the Software, and to prepare derivative works of the
Software, and to permit third-parties to whom the Software is furnished to
do so, all subject to the following:

The copyright notices in the Software and this entire statement, including
the above license grant, this restriction and the following disclaimer,
must be included in all copies of the Software, in whole or in part, and
all derivative works of the Software, unless such copies or derivative
works are solely in the form of machine-executable object code generated by
a source language processor.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT
SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE
FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE."""


def fail(message):
    sys.exit(f"third_party_notices: {message}")


def spdx_ids(expression):
    """The license identifiers in a Cargo `license` field."""
    expression = expression.replace("/", " OR ")
    ids = []
    for token in re.split(r"[\s()]+", expression):
        if token and token not in ("OR", "AND", "WITH"):
            ids.append(ALIASES.get(token, token))
    return ids


def normalize(text):
    lines = [line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    return "\n".join(lines).strip("\n")


def read_text(path):
    with open(path, "rb") as f:
        return normalize(f.read().decode("utf-8", errors="replace"))


def metadata(workspace, target):
    result = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1", "--filter-platform", target],
        cwd=os.path.join(ROOT, workspace),
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        fail(f"cargo metadata failed in {workspace} for {target}:\n{result.stderr}")
    return json.loads(result.stdout)


def shipped_packages():
    """{(name, version): package} for every crate a shipped build links."""
    found = {}
    for workspace, roots, targets in WORKSPACES:
        for target in targets:
            meta = metadata(workspace, target)
            packages = {p["id"]: p for p in meta["packages"]}
            nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
            stack = [p["id"] for p in meta["packages"] if p["name"] in roots]
            seen = set()
            while stack:
                pid = stack.pop()
                if pid in seen:
                    continue
                seen.add(pid)
                for dep in nodes[pid]["deps"]:
                    if any(kind["kind"] is None for kind in dep["dep_kinds"]):
                        stack.append(dep["pkg"])
            for pid in seen:
                package = packages[pid]
                if package["name"] in OURS:
                    continue
                key = (package["name"], package["version"])
                # Prefer a registry or git copy over a local path patch of the
                # same crate (the web workspace's patched NIH-plug).
                if key not in found or found[key]["source"] is None:
                    found[key] = package
    return found


def license_files(directory):
    names = sorted(
        f for f in os.listdir(directory)
        if LICENSE_FILE.match(f) and os.path.isfile(os.path.join(directory, f))
    )
    return [os.path.join(directory, f) for f in names]


def repository_root(directory):
    """The checkout or vendored repository a crate directory sits in, if any."""
    current = directory
    for _ in range(4):
        parent = os.path.dirname(current)
        if parent == current:
            return None
        current = parent
        if os.path.exists(os.path.join(current, ".git")) or os.path.exists(
            os.path.join(current, ".cargo-ok")
        ):
            return current
    return None


def root_license(directory):
    manifest = os.path.join(directory, "Cargo.toml")
    if not os.path.isfile(manifest):
        return None
    with open(manifest, encoding="utf-8") as f:
        match = re.search(r'^license\s*=\s*"([^"]+)"', f.read(), re.MULTILINE)
    return match.group(1) if match else None


def holders(package):
    authors = [re.sub(r"\s*<[^>]*>", "", a).strip() for a in package.get("authors") or []]
    authors = [a for a in authors if a]
    return ", ".join(authors) if authors else f"the {package['name']} authors"


def standard_text(package):
    """(text, how) for a crate published without a license file."""
    ids = spdx_ids(package["license"])
    if "MIT" in ids:
        return MIT_TEMPLATE.format(holders=holders(package)), "standard MIT text"
    if "ISC" in ids:
        return ISC_TEMPLATE.format(holders=holders(package)), "standard ISC text"
    if "BSL-1.0" in ids:
        return BSL_TEXT, "standard BSL-1.0 text"
    fail(f"{package['name']} {package['version']} has no license file and no standard text for {package['license']}")


def texts_for(package):
    """[(label, text)] and a note on where they came from."""
    directory = os.path.dirname(package["manifest_path"])
    files = license_files(directory)
    for extra in EXTRA_FILES.get(package["name"], []):
        path = os.path.join(directory, extra)
        if not os.path.isfile(path):
            fail(f"{package['name']}: expected license file {extra} is missing")
        files.append(path)
    if files:
        return [(os.path.relpath(f, directory), read_text(f)) for f in files], None
    root = repository_root(directory)
    if root and root_license(root) == package["license"]:
        files = license_files(root)
        if files:
            return [(os.path.basename(f), read_text(f)) for f in files], "from its repository's root"
    text, how = standard_text(package)
    return [("license", text)], f"{how}; the published crate ships no license file"


def collapse(text):
    return re.sub(r"\s+", " ", text)


APACHE_END = "END OF TERMS AND CONDITIONS"


def apache_terms(text):
    """The Apache License's terms without its appendix.

    Copies differ only in the appendix (how to apply the license, often with
    the template's placeholders) and in http vs https; one copy of the terms is
    what the license asks to be given with the binaries.
    """
    if text.lstrip().startswith("Apache License") and APACHE_END in text:
        return text[: text.index(APACHE_END) + len(APACHE_END)].strip("\n")
    return text


def kind_of(text):
    text = collapse(text)
    head = text[:600]
    if "Apache License" in head:
        return "Apache License 2.0"
    if "Boost Software License" in head:
        return "Boost Software License 1.0"
    if "SIL OPEN FONT LICENSE" in head.upper():
        return "SIL Open Font License 1.1"
    if "UBUNTU FONT LICENCE" in head.upper():
        return "Ubuntu Font Licence 1.0"
    if "GNU GENERAL PUBLIC LICENSE" in head.upper():
        return "GNU General Public License"
    if "UNICODE" in head.upper():
        return "Unicode License"
    if "This is free and unencumbered software" in head:
        return "The Unlicense"
    if "zlib" in head.lower() or "In no event will the authors be held liable" in text:
        return "zlib License"
    if "Permission to use, copy, modify, and/or distribute" in text:
        return "ISC License"
    if "Permission is hereby granted, free of charge, to any person obtaining" in text:
        return "MIT License"
    if "Redistribution and use in source and binary forms" in text:
        return "BSD License"
    return "License text"


def source_link(package):
    repository = package.get("repository")
    if repository:
        return repository.removesuffix(".git")
    source = package.get("source") or ""
    if source.startswith("registry+"):
        return f"https://crates.io/crates/{package['name']}"
    if source.startswith("git+"):
        return source[4:].split("?")[0].split("#")[0].removesuffix(".git")
    return ""


def fence(text):
    return f"````text\n{text}\n````"


def generate():
    packages = shipped_packages()
    problems = []
    rows = []
    # Texts that differ only in whitespace share a group; the first one (in
    # crate order) is printed.
    groups = {}  # collapsed text -> {"text": str, "crates": [str]}
    notes = []
    for key in sorted(packages):
        package = packages[key]
        name, version = key
        license_expr = package.get("license") or ""
        if not license_expr:
            problems.append(f"{name} {version}: no license field")
            continue
        if name in FORBIDDEN:
            problems.append(f"{name} {version} must not ship: {FORBIDDEN[name]}")
        unknown = [i for i in spdx_ids(license_expr) if i not in ALLOWED]
        if unknown:
            problems.append(f"{name} {version}: {license_expr} (not on the allowlist: {', '.join(unknown)})")
        link = source_link(package)
        rows.append(f"| {name} | {version} | {license_expr} | {link} |")
        texts, note = texts_for(package)
        if note:
            notes.append(f"- {name} {version}: {note}.")
        for _, text in texts:
            text = apache_terms(text)
            key = collapse(text).replace("https://", "http://").strip()
            group = groups.setdefault(key, {"text": text, "crates": []})
            crate = f"{name} {version}"
            if crate not in group["crates"]:
                group["crates"].append(crate)
    if problems:
        fail("licenses need a look before they ship:\n  " + "\n  ".join(problems))

    out = [BEGIN, ""]
    out.append(
        f"{len(rows)} crates. Regenerate with `python3 scripts/third_party_notices.py`; "
        "CI fails while this section is stale."
    )
    out += ["", "### Crates", "", "| Crate | Version | License | Source |", "|---|---|---|---|"]
    out += rows
    if notes:
        out += ["", "### Crates published without a license file", ""]
        out.append(
            "Their texts below come from their repository or are the standard text of a "
            "license they offer, with the authors their Cargo.toml lists as copyright holders."
        )
        out.append("")
        out += notes
    out += ["", "### License texts", ""]
    out.append(
        "Each text appears once, followed by the crates it covers. The Apache License 2.0 "
        "is given without its appendix (how to apply the license), in which copies differ."
    )
    ordered = sorted(groups.values(), key=lambda group: (kind_of(group["text"]), group["crates"][0]))
    for group in ordered:
        out += ["", f"#### {kind_of(group['text'])}", ""]
        out.append("Used by: " + ", ".join(group["crates"]) + ".")
        out += ["", fence(group["text"])]
    out += ["", END]
    return "\n".join(out)


def main():
    check = "--check" in sys.argv[1:]
    subprocess.run(
        [os.path.join(ROOT, "web", "scripts", "fetch-nih-plug.sh")],
        check=True,
        stdout=subprocess.DEVNULL,
    )
    with open(NOTICES, encoding="utf-8") as f:
        current = f.read()
    if BEGIN not in current or END not in current:
        fail(f"{NOTICES} lacks the GENERATED markers")
    before, rest = current.split(BEGIN, 1)
    _, after = rest.split(END, 1)
    updated = before + generate() + after
    if check:
        if updated != current:
            fail(
                "THIRD-PARTY-NOTICES.md is stale: the shipped crates or their licenses changed. "
                "Run `python3 scripts/third_party_notices.py` and commit the result."
            )
        print("THIRD-PARTY-NOTICES.md is current")
        return
    with open(NOTICES, "w", encoding="utf-8", newline="\n") as f:
        f.write(updated)
    print(f"wrote {os.path.relpath(NOTICES, ROOT)}")


if __name__ == "__main__":
    main()
