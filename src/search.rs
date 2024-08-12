use gtk::{glib, gdk, gio, prelude::*};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use crate::entries::LauncherEntry;

struct SearchResult {
    score: i64,
    entry: LauncherEntry
}

pub fn handle_search(query: &str, launcher_entries: &Vec<LauncherEntry>) -> Vec<LauncherEntry> {
    let mut results: Vec<SearchResult> = Vec::new();
    let matcher = SkimMatcherV2::default();

    for entry in launcher_entries {
        let matches = matcher.fuzzy_match(&entry.keywords, query);
        if matches.is_some() {
            let score = matches.unwrap();
            // TODO: Figure out minimum score
            if score > 0 {
                results.push(SearchResult {
                    score,
                    entry: entry.clone()
                });
            }
        }
    }
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results.into_iter().map(|result| result.entry).collect()
}
