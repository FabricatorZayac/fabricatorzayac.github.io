use std::{fs::{self, File}, io::Write, path::Path};

use maud::{html, PreEscaped, DOCTYPE};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let asset_paths: Vec<_> = WalkDir::new("assets").into_iter()
        .filter_map(|e| e.ok())
        .filter(is_md_file)
        .map(|e| DirEntry::path(&e).to_owned())
        .collect();

    println!("{:?}", asset_paths);

    let html_paths: Vec<_> = asset_paths.iter()
        .map(|mdpath| mdpath.with_extension("html"))
        .map(|htmlpath| Path::new("site").join(htmlpath.strip_prefix("assets").unwrap()))
        .collect();

    println!("{:?}", html_paths);

    for (mdpath, htmlpath) in asset_paths.iter().zip(html_paths.iter()) {
        let html = html! {
            (DOCTYPE)
            head {
                link rel="stylesheet" href="/style.css";
            }
            body {
                div { 
                    form {
                        button formaction="/" class="button-54" { "Home" }
                    }
                }
                (parse_markdown(&fs::read_to_string(mdpath).unwrap()))
                script { "history.replaceState({page: 1}, '', window.location.pathname.replace('.html', ''))" }
            }
        };
        fs::create_dir_all(htmlpath.parent().unwrap()).unwrap();
        let mut file = File::create(htmlpath).unwrap();
        file.write_all(html.into_string().as_bytes()).unwrap();
    }
}

fn is_md_file(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.ends_with(".md"))
        .unwrap_or(false)
}

fn parse_markdown(md: &str) -> PreEscaped<String> {
    let mut options = comrak::Options::default();
    options.extension.underline = true;
    options.extension.strikethrough = true;
    options.render.unsafe_ = true;

    let adapter = comrak::plugins::syntect::SyntectAdapterBuilder::new()
        .theme("InspiredGitHub")
        .build();

    let mut plugins = comrak::Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    PreEscaped(comrak::markdown_to_html_with_plugins(md, &options, &plugins))
}
