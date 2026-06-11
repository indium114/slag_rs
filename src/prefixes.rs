use colored::*;

pub static ok_text: ColoredString = "[ok]".green();
pub static warn_text: ColoredString = "[warn]".yellow();
pub static err_text: ColoredString = "[err]".red();
pub static sync_text: ColoredString = "[sync]".blue();
pub static update_text: ColoredString = "[update]".magenta();
pub static log_text: ColoredString = "[log]".black().bold();
pub static clean_text: ColoredString = "[clean]".cyan();
pub static add_text: ColoredString = "[add]".magenta().bold();
pub static hint_text: ColoredString = "[hint]".cyan().bold();
pub static query_text: ColoredString = "[query]".yellow().bold();
