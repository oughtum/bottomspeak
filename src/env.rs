use std::env;

use rand::{rngs::SmallRng, seq::IndexedRandom};

pub(crate) const DEFAULT_INTERP_TITLES: [&str; 1] = ["mommy"];
pub(crate) const DEFAULT_PETNAMES: [&str; 5] =
    ["sweetheart", "sweetie", "cutie", "darling", "honey"];
pub(crate) const DEFAULT_PRAISE_TERMS: [&str; 2] = ["girl", "pet"];
pub(crate) const DEFAULT_PRINT_KEYWORDS: [&str; 6] =
    ["meow", "mreow", "mrrp", "woof", "wruff", "yip"];

pub(crate) struct EnvVars {
    pub(crate) interp_titles: Vec<String>,
    pub(crate) petnames: Vec<String>,
    pub(crate) praise_terms: Vec<String>,
    pub(crate) print_keywords: Vec<String>,
}

impl EnvVars {
    pub(crate) fn new() -> Self {
        Self {
            interp_titles: get_env_strings(
                "BOTTOMSPEAK_INTERPRETER_TITLES",
                &DEFAULT_INTERP_TITLES,
            ),
            petnames: get_env_strings("BOTTOMSPEAK_PETNAMES", &DEFAULT_PETNAMES),
            praise_terms: get_env_strings("BOTTOMSPEAK_PRAISE_TERMS", &DEFAULT_PRAISE_TERMS),
            print_keywords: get_env_strings("BOTTOMSPEAK_PRINT_KEYWORDS", &DEFAULT_PRINT_KEYWORDS),
        }
    }

    /// Samples a random interpreter name.
    pub(crate) fn rand_interp_title(&self) -> &str {
        let mut rng = rand::make_rng::<SmallRng>();
        self.interp_titles.choose(&mut rng).unwrap()
    }

    /// Samples a random petname.
    pub(crate) fn rand_petname(&self) -> &str {
        let mut rng = rand::make_rng::<SmallRng>();
        self.petnames.choose(&mut rng).unwrap()
    }

    /// Samples a random praise term.
    pub(crate) fn rand_praise_term(&self) -> &str {
        let mut rng = rand::make_rng::<SmallRng>();
        self.praise_terms.choose(&mut rng).unwrap()
    }
}

fn get_env_strings(env_var: &str, defaults: &[&str]) -> Vec<String> {
    env::var(env_var)
        .map(|list| list.split('/').map(String::from).collect::<Vec<String>>())
        .unwrap_or_else(|_| defaults.iter().copied().map(String::from).collect())
}
