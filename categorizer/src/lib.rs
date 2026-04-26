#[derive(Debug, Default)]
pub enum CategoryMethod {
    #[default]
    None,
    Regex(fn (&str) -> CategoryMatch),
    Cosine(fn (&str) -> CategoryMatch),
    LLM(fn (&str) -> CategoryMatch),
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Confidence {
    pub min:f32,
    pub max:f32,
    pub actual:f32,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct CategoryMatch {
    pub yes: bool,
    pub confidence: Confidence
}

#[derive(Debug, Default)]
pub struct Category {
    pub id: u32,
    pub label: String,
    pub action: CategoryMethod,
    pub order: u32,
    pub group: u32,
}

impl CategoryMatch {
    pub fn is_other_better(&self,other:&CategoryMatch) -> bool {
        other.yes == self.yes && other.confidence > self.confidence
    }
}

pub fn make_regex_action(regex:&str,confidence:f32) -> impl Fn (&str) -> CategoryMatch {
    let re = regex::Regex::new(regex).unwrap();

    move |msg:&str| {
        CategoryMatch { 
            yes: re.is_match(msg), 
            confidence: Confidence { 
                min: confidence, 
                max:confidence, 
                actual: confidence 
            } 
        }
    }
}