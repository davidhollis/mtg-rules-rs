use std::error::Error;
use std::io::{self, prelude::*};

use rules::Rule;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let current_cr = rules::parser::parse(&buffer);

    println!("Rules structure, effective {}", current_cr.effective_date);
    println!("");

    print_outline(&current_cr.rules, 0);

    Ok(())
}

fn print_outline(rules: &[Rule], indent: usize) {
    let prefix = std::iter::repeat(" ").take(indent).collect::<String>();
    for rule in rules {
        let rule_width = 120 - indent - rule.id.len() - 5;
        if rule.text.chars().count() > rule_width {
            let truncated = rule.text.chars().take(rule_width - 3).collect::<String>();
            println!("{}- [{}] {}...", prefix, rule.id, truncated);
        } else {
            println!("{}- [{}] {}", prefix, rule.id, rule.text);
        }
        print_outline(&rule.subrules, indent + 4);
    }
}