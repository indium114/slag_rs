use colored::*;
use std::fmt;

/// # usefulog
/// a simple logging library for Rust
/// all of the functions in this crate allow you to print logs with *style*!
/// it adds a colourful prefix to each printed line.

// MARK: reusable print function
fn generic_print(prefix: ColoredString, text: impl fmt::Display) {
    println!("{prefix} {text}")
}

// MARK: actual log thingies.

/// prints `text` with the `[ok]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::ok;
/// usefulog::ok("hello")
/// // outputs `[ok] hello`
/// ```
pub fn ok(text: impl fmt::Display) {
    generic_print("[ok]".green(), text);
}

/// prints `text` with the `[warn]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::warn;
/// usefulog::warn("hello")
/// // outputs `[warn] hello`
/// ```
pub fn warn(text: impl fmt::Display) {
    generic_print("[warn]".yellow(), text);
}

/// prints `text` with the `[err]` prefix
/// this one's special, as it prints to stderr (`eprintln!()`)
///
/// # examples
///
/// ```rust
/// use usefulog::err;
/// usefulog::err("hello")
/// // outputs `[err] hello`
/// ```
pub fn err(text: impl fmt::Display) {
    let prefix: ColoredString = "[err]".red();
    eprintln!("{prefix} {text}")
}

/// prints `text` with the `[sync]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::sync;
/// usefulog::sync("hello")
/// // outputs `[sync] hello`
/// ```
pub fn sync(text: impl fmt::Display) {
    generic_print("sync".blue(), text)
}

/// prints `text` with the `[update]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::update;
/// usefulog::update("hello")
/// // outputs `[update] hello`
/// ```
pub fn update(text: impl fmt::Display) {
    generic_print("[update]".magenta(), text);
}

/// prints `text` with the `[log]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::log;
/// usefulog::log("hello")
/// // outputs `[log] hello`
/// ```
pub fn log(text: impl fmt::Display) {
    generic_print("[log]".black().bold(), text);
}

/// prints `text` with the `[clean]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::clean;
/// usefulog::clean("hello")
/// // outputs `[clean] hello`
/// ```
pub fn clean(text: impl fmt::Display) {
    generic_print("[clean]".cyan(), text);
}

/// prints `text` with the `[add]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::add;
/// usefulog::add("hello")
/// // outputs `[add] hello`
/// ```
pub fn add(text: impl fmt::Display) {
    generic_print("[add]".magenta().bold(), text);
}

/// prints `text` with the `[hint]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::hint;
/// usefulog::hint("hello")
/// // outputs `[hint] hello`
/// ```
pub fn hint(text: impl fmt::Display) {
    generic_print("[hint]".cyan().bold(), text);
}

/// prints `text` with the `[query]` prefix
///
/// # examples
///
/// ```rust
/// use usefulog::query;
/// usefulog::query("hello")
/// // outputs `[query] hello`
/// ```
pub fn query(text: impl fmt::Display) {
    generic_print("[query]".yellow().bold(), text);
}
