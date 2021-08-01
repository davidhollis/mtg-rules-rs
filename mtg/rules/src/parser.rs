use regex::Regex;
use lazy_static::lazy_static;

use crate::{Rule, Edition};

lazy_static! {
    static ref RX_EFFECTIVE_DATE: Regex = Regex::new(
        "\\AThese rules are effective as of (?P<effective_date>.+)\\.\\z"
    ).unwrap();
    static ref RX_RULE_LINE: Regex = Regex::new(
        "\\A(?P<rule_id>\\d(?:\\d{2}(?:\\.\\d+[a-z]?)?)?)\\.? (?P<rule_text>.+)\\z"
    ).unwrap();
    static ref RX_EXAMPLE_LINE: Regex = Regex::new(
        "\\AExample: (?P<example_text>.+)\\z"
    ).unwrap();
}

enum Section {
    Heading,
    Introduction,
    TableOfContents,
    Rules,
    Glossary,
    Credits,
}

struct ParserState {
    edition: Edition,
    current_rules: Vec<Rule>,
    current_term: Option<String>,
    current_section: Section,
}

impl ParserState {
    fn new() -> ParserState {
        ParserState {
            edition: Edition::new(),
            current_rules: vec![],
            current_term: None,
            current_section: Section::Heading,
        }
    }

    fn update(&mut self, line: &str) {
        match &self.current_section {
            Section::Heading => {
                if let Some(date_capture) = RX_EFFECTIVE_DATE.captures(line) {
                    self.edition.effective_date =
                        date_capture.name("effective_date").map_or("", |m| m.as_str()).to_string();
                } else if line == "Introduction" {
                    self.current_section = Section::Introduction;
                }
            },
            Section::Introduction => {
                if line == "Contents" {
                    self.current_section = Section::TableOfContents;
                } else {
                    self.edition.introduction.push_str(line);
                    self.edition.introduction.push('\n');
                }
            }
            Section::TableOfContents => {
                if line == "Credits" {
                    self.current_section = Section::Rules;
                }
            },
            Section::Rules => {
                if line == "Glossary" {
                    self.roll_up_until("");
                    self.current_section = Section::Glossary;
                } else if let Some(rule_capture) = RX_RULE_LINE.captures(line) {
                    // rule
                    let rule_id = rule_capture.name("rule_id").map_or("", |m| m.as_str()).to_string();
                    let rule_text = rule_capture.name("rule_text").map_or("", |m| m.as_str()).to_string();
                    self.roll_up_until(&rule_id);
                    self.current_rules.push(
                        Rule::new(
                            rule_id,
                            rule_text,
                        )
                    );
                } else if let Some(example_capture) = RX_EXAMPLE_LINE.captures(line) {
                    let example_text =
                        example_capture.name("example_text").map_or("", |m| m.as_str()).to_string();

                    if let Some(current_rule) = self.current_rules.last_mut() {
                        current_rule.examples.push(example_text);
                    }
                }
            },
            Section::Glossary => {
                if line.is_empty() {
                    // blank line is the boundary between terms
                    self.current_term = None;
                } else if line == "Credits" {
                    self.current_section = Section::Credits;
                } else if let Some(term) = self.current_term.take() {
                    self.edition.glossary.insert(term, line.to_string());
                } else {
                    self.current_term = Some(line.to_string());
                }
            },
            Section::Credits => {
                self.edition.credits.push_str(line);
                self.edition.credits.push('\n');
            },
        }
    }

    fn roll_up_until(&mut self, rule_id: &str) {
        if let Some(top_rule) = self.current_rules.pop() {
            // pop the top rule off the stack
            if rule_id.starts_with(&top_rule.id) {
                // if the top rule on the stack is a parent of the rule in question, put it back
                self.current_rules.push(top_rule);
            } else {
                // if the top rule on the stack is NOT a parent of the rule in question,
                // grab the _next_ rule on the stack
                if let Some(parent_rule) = self.current_rules.last_mut() {
                    // if there is a parent rule, add the previous top rule to it and keep rolling upward
                    parent_rule.subrules.push(top_rule);
                    self.roll_up_until(rule_id);
                } else {
                    // if there is no parent rule, add the previous rule to the edition
                    self.edition.rules.push(top_rule);
                }
            }
        }
    }

    fn finalize(self) -> Edition {
        self.edition
    }
}

pub fn parse(document: &str) -> Edition {
    let mut parser_state = ParserState::new();
    
    for line in document.lines() {
        parser_state.update(line);
    }

    parser_state.finalize()
}