use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
const AMBIGUOUS_CHARS: &str = "0O1lI";

const WORD_LIST: &[&str] = &[
    "apple", "banana", "cherry", "dragon", "eagle", "falcon", "guitar", "hammer", "island",
    "jungle", "kingdom", "lemon", "marble", "nectar", "orange", "puzzle", "quartz", "river",
    "sunset", "thunder", "umbrella", "violin", "winter", "xenon", "yellow", "zephyr", "anchor",
    "beacon", "castle", "diamond", "ember", "forest", "glacier", "harbor", "ivory", "jasper",
    "knight", "lantern", "meadow", "noble", "ocean", "phoenix", "quantum", "rapids", "silver",
    "temple", "unity", "valley", "wonder", "crystal", "breeze", "cosmos", "dusk", "flame", "grove",
    "horizon", "jade", "karma", "lotus", "mist", "nova", "orbit", "prism", "quest", "reef",
    "storm", "tide", "vine", "wave", "zen", "aurora", "blaze", "cliff", "delta", "echo", "frost",
    "glow", "haven", "ink", "jewel", "kite", "lunar", "maple", "north", "opal", "pearl", "raven",
    "spark", "terra", "ultra", "vivid", "wisp",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOptions {
    pub length: u32,
    pub include_lowercase: bool,
    pub include_uppercase: bool,
    pub include_digits: bool,
    pub include_symbols: bool,
    pub exclude_ambiguous: bool,
    pub custom_exclude: String,
    pub count: u32,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            length: 16,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: true,
            exclude_ambiguous: false,
            custom_exclude: String::new(),
            count: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PassphraseOptions {
    pub word_count: u32,
    pub separator: String,
    pub capitalize: bool,
    pub include_number: bool,
    pub count: u32,
}

impl Default for PassphraseOptions {
    fn default() -> Self {
        Self {
            word_count: 4,
            separator: "-".to_string(),
            capitalize: true,
            include_number: true,
            count: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPassword {
    pub value: String,
    pub strength: PasswordStrength,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordStrength {
    pub score: u32,
    pub label: String,
    pub entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordGenerateResult {
    pub success: bool,
    pub passwords: Vec<GeneratedPassword>,
    pub error: Option<String>,
}

fn build_charset(options: &PasswordOptions) -> String {
    let mut charset = String::new();

    if options.include_lowercase {
        charset.push_str(LOWERCASE);
    }
    if options.include_uppercase {
        charset.push_str(UPPERCASE);
    }
    if options.include_digits {
        charset.push_str(DIGITS);
    }
    if options.include_symbols {
        charset.push_str(SYMBOLS);
    }

    if options.exclude_ambiguous {
        charset = charset
            .chars()
            .filter(|c| !AMBIGUOUS_CHARS.contains(*c))
            .collect();
    }

    if !options.custom_exclude.is_empty() {
        charset = charset
            .chars()
            .filter(|c| !options.custom_exclude.contains(*c))
            .collect();
    }

    charset
}

fn calculate_strength(password: &str, charset_size: usize) -> PasswordStrength {
    let length = password.len();
    let entropy = (length as f64) * (charset_size as f64).log2();

    let (score, label) = if entropy < 28.0 {
        (1, "非常に弱い")
    } else if entropy < 36.0 {
        (2, "弱い")
    } else if entropy < 60.0 {
        (3, "普通")
    } else if entropy < 80.0 {
        (4, "強い")
    } else {
        (5, "非常に強い")
    };

    PasswordStrength {
        score,
        label: label.to_string(),
        entropy: (entropy * 100.0).round() / 100.0,
    }
}

pub fn generate_passwords(options: PasswordOptions) -> PasswordGenerateResult {
    let charset = build_charset(&options);

    if charset.is_empty() {
        return PasswordGenerateResult {
            success: false,
            passwords: vec![],
            error: Some("文字種を1つ以上選択してください".to_string()),
        };
    }

    let charset_chars: Vec<char> = charset.chars().collect();
    let charset_size = charset_chars.len();
    let length = options.length.clamp(4, 128) as usize;
    let count = options.count.clamp(1, 100);

    let mut rng = rand::thread_rng();
    let passwords: Vec<GeneratedPassword> = (0..count)
        .map(|_| {
            let password: String = (0..length)
                .map(|_| {
                    let idx = rng.gen_range(0..charset_size);
                    charset_chars[idx]
                })
                .collect();

            let strength = calculate_strength(&password, charset_size);
            GeneratedPassword {
                value: password,
                strength,
            }
        })
        .collect();

    PasswordGenerateResult {
        success: true,
        passwords,
        error: None,
    }
}

pub fn generate_passphrases(options: PassphraseOptions) -> PasswordGenerateResult {
    let word_count = options.word_count.clamp(2, 10) as usize;
    let count = options.count.clamp(1, 100);

    let mut rng = rand::thread_rng();
    let passwords: Vec<GeneratedPassword> = (0..count)
        .map(|_| {
            let mut words: Vec<String> = WORD_LIST
                .choose_multiple(&mut rng, word_count)
                .map(|w| {
                    if options.capitalize {
                        let mut chars = w.chars();
                        match chars.next() {
                            Some(first) => first.to_uppercase().chain(chars).collect(),
                            None => String::new(),
                        }
                    } else {
                        w.to_string()
                    }
                })
                .collect();

            if options.include_number {
                let num = rng.gen_range(0..100);
                words.push(num.to_string());
            }

            let passphrase = words.join(&options.separator);

            let pool_size = WORD_LIST.len();
            let entropy = (word_count as f64) * (pool_size as f64).log2()
                + if options.include_number {
                    100_f64.log2()
                } else {
                    0.0
                };

            let (score, label) = if entropy < 40.0 {
                (2, "弱い")
            } else if entropy < 60.0 {
                (3, "普通")
            } else if entropy < 80.0 {
                (4, "強い")
            } else {
                (5, "非常に強い")
            };

            let strength = PasswordStrength {
                score,
                label: label.to_string(),
                entropy: (entropy * 100.0).round() / 100.0,
            };

            GeneratedPassword {
                value: passphrase,
                strength,
            }
        })
        .collect();

    PasswordGenerateResult {
        success: true,
        passwords,
        error: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let options = PasswordOptions {
            length: 16,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: false,
            exclude_ambiguous: false,
            custom_exclude: String::new(),
            count: 1,
        };

        let result = generate_passwords(options);
        assert!(result.success);
        assert_eq!(result.passwords.len(), 1);
        assert_eq!(result.passwords[0].value.len(), 16);
    }

    #[test]
    fn test_generate_multiple_passwords() {
        let options = PasswordOptions {
            length: 12,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: true,
            exclude_ambiguous: false,
            custom_exclude: String::new(),
            count: 5,
        };

        let result = generate_passwords(options);
        assert!(result.success);
        assert_eq!(result.passwords.len(), 5);
    }

    #[test]
    fn test_exclude_ambiguous() {
        let options = PasswordOptions {
            length: 100,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: false,
            exclude_ambiguous: true,
            custom_exclude: String::new(),
            count: 1,
        };

        let result = generate_passwords(options);
        assert!(result.success);
        let password = &result.passwords[0].value;
        assert!(!password.contains('0'));
        assert!(!password.contains('O'));
        assert!(!password.contains('l'));
        assert!(!password.contains('1'));
        assert!(!password.contains('I'));
    }

    #[test]
    fn test_generate_passphrase() {
        let options = PassphraseOptions {
            word_count: 4,
            separator: "-".to_string(),
            capitalize: true,
            include_number: true,
            count: 1,
        };

        let result = generate_passphrases(options);
        assert!(result.success);
        assert_eq!(result.passwords.len(), 1);
        assert!(result.passwords[0].value.contains('-'));
    }

    #[test]
    fn test_empty_charset() {
        let options = PasswordOptions {
            length: 16,
            include_lowercase: false,
            include_uppercase: false,
            include_digits: false,
            include_symbols: false,
            exclude_ambiguous: false,
            custom_exclude: String::new(),
            count: 1,
        };

        let result = generate_passwords(options);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_password_strength() {
        let options = PasswordOptions {
            length: 32,
            include_lowercase: true,
            include_uppercase: true,
            include_digits: true,
            include_symbols: true,
            exclude_ambiguous: false,
            custom_exclude: String::new(),
            count: 1,
        };

        let result = generate_passwords(options);
        assert!(result.success);
        assert!(result.passwords[0].strength.score >= 4);
    }
}
