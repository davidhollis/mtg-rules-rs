pub struct Corpus {
    pub documents: Vec<Document>,
}

impl Corpus {
    pub fn lookup(&self, doc_name: &str, rule: &str) -> Option<&Rule> {
        let doc = self.documents.iter().find(|d| d.name == doc_name)?;
        doc.lookup(rule)
    }
}

pub struct Document {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Document {
    pub fn lookup(&self, rule: &str) -> Option<&Rule> {
        Rule::find_recursive(&self.rules, rule)
    }
}

pub struct Rule {
    pub id: String,
    pub text: String,
    pub subrules: Vec<Rule>,
    pub examples: Vec<String>,
}

impl Rule {
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
