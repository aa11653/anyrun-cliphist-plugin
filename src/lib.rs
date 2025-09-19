use std::fs;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::Deserialize;

use crate::cliphist::wipe_history;
mod cliphist;

#[derive(Deserialize)]
struct Config {
    // Maximum number of clipboard entries to display
    max_entries: usize,
    // Whether to show descriptions for each entry
    show_description: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_entries: 10,
            show_description: true,
        }
    }
}

/// State of the plugin
struct State {
    history: Result<Vec<cliphist::ClipboardEntry>, cliphist::CliphistError>,
    config: Config,
}

/// Special match ID to indicate an error state
const OPTION_ERROR: u64 = u64::MAX;
/// Spetial match ID to indicate wiping the clipboard history
const OPTION_WIPE: u64 = u64::MAX - 1;

#[init]
fn init(config_dir: RString) -> State {
    State {
        history: cliphist::get_history(),
        config: match fs::read_to_string(format!("{}/cliphist.ron", config_dir)) {
            Ok(content) => ron::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        },
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Clipboard".into(),
        icon: "edit-copy".into(), // Icon from the icon theme
    }
}

/// Helper function to create an optional description
/// based on the `show` flag.
///
/// Parameters:
/// - `msg`: The message to include in the description.
/// - `show`: A boolean flag indicating whether to show the description.
fn make_description(msg: &str, show: bool) -> ROption<RString> {
    if show {
        ROption::RSome(msg.into())
    } else {
        ROption::RNone
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    match state {
        State {
            history: Err(err), ..
        } => {
            return vec![Match {
                title: format!("Error: {}", err).into(),
                description: make_description(
                    "Error encountered calling cliphist",
                    state.config.show_description,
                ),
                use_pango: false,
                icon: ROption::RSome("dialog-error".into()),
                id: ROption::RSome(OPTION_ERROR.into()),
            }]
            .into();
        }
        State {
            history: Ok(entries),
            ..
        } => {
            let matcher = SkimMatcherV2::default();
            let mut matches: Vec<(i64, &cliphist::ClipboardEntry)> = entries
                .iter()
                .filter_map(|entry| {
                    if input.is_empty() {
                        // If input is empty, include all entries with a default score of 0
                        return Some((0, entry));
                    }
                    matcher
                        .fuzzy_match(&entry.content, &input)
                        .map(|score| (score, entry))
                })
                .collect();

            matches.sort_by(|a, b| {
                // sort by score, then index
                b.0.cmp(&a.0).then_with(|| b.1.index.cmp(&a.1.index))
            });

            matches.truncate(state.config.max_entries);

            let mut result: Vec<Match> = matches
                .into_iter()
                .map(|(_, entry)| Match {
                    title: entry.content.clone().into(),
                    description: make_description(
                        format!("Entry {}", entry.index.to_string()).as_str(),
                        state.config.show_description,
                    ),
                    use_pango: false,
                    icon: ROption::RNone,
                    id: ROption::RSome(entry.index.into()),
                })
                .collect();

            // Add a special option to wipe the clipboard history
            result.push(Match {
                title: "<i>** Wipe Clipboard History</i>".into(),
                description: make_description("clear history", state.config.show_description),
                use_pango: true,
                icon: ROption::RNone,
                id: ROption::RSome(OPTION_WIPE.into()),
            });
            return result.into();
        }
    }
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    match selection.id {
        ROption::RSome(OPTION_ERROR) => {
            // Do nothing for error option
            HandleResult::Close
        }
        ROption::RSome(OPTION_WIPE) => {
            // Wipe clipboard history
            if let Err(e) = wipe_history() {
                return HandleResult::Stdout(
                    format!("Failed to wipe clipboard history: {}", e)
                        .as_str()
                        .into(),
                );
            }
            HandleResult::Close
        }
        ROption::RSome(id) => {
            let id: u64 = id.into();
            match cliphist::copy_history(id) {
                Ok(_) => HandleResult::Close,
                Err(e) => HandleResult::Stdout(
                    format!("Failed to select clipboard entry: {}", e)
                        .as_str()
                        .into(),
                ),
            }
        }
        ROption::RNone => {
            // Do nothing if no ID is provided
            HandleResult::Close
        }
    }
}
