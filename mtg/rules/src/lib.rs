use std::collections::HashMap;

pub mod parser;

pub struct Edition {
    pub effective_date: String,
    pub introduction: String,
    pub rules: Vec<Rule>,
    pub glossary: HashMap<String, String>,
    pub credits: String,
}

impl Edition {
    pub fn new() -> Edition {
        Edition {
            effective_date: "August 5, 1993".to_string(),
            introduction: String::new(),
            rules: vec![],
            glossary: HashMap::new(),
            credits: String::new(),
        }
    }
    pub fn lookup(&self, rule: &str) -> Option<&Rule> {
        Rule::find_recursive(&self.rules, rule)
    }
}

pub struct Rule {
    pub id: String,
    pub text: String,
    pub subrules: Vec<Rule>,
    pub examples: Vec<String>,
    pub renumbered_from: Option<String>,
}

impl Rule {
    pub fn new(id: String, text: String) -> Rule {
        Rule {
            id: id,
            text: text,
            subrules: vec![],
            examples: vec![],
            renumbered_from: None,
        }
    }

    pub fn lookup(&self, id: &str) -> Option<&Rule> {
        if self.id == id {
            Some(self)
        } else {
            Rule::find_recursive(&self.subrules, id)
        }
    }

    fn find_prefix<'a>(rules: &'a [Rule], id: &str) -> Option<&'a Rule> {
        rules.iter().find(|r| id.starts_with(&r.id))
    }

    fn find_recursive<'a>(rules: &'a [Rule], id: &str) -> Option<&'a Rule> {
        let first_match = Rule::find_prefix(rules, id)?;
        if first_match.id == id {
            Some(first_match)
        } else {
            Rule::find_recursive(&first_match.subrules, id)
        }
    }
}
