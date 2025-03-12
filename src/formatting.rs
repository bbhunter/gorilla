use crate::mutation::{MutationResult, MutationSet};

pub struct FormatPart {
    pub text: String,
    pub mutations: MutationSet,
}

pub struct FormatSet {
    pub parts: Vec<FormatPart>,
}

pub struct FormatField {
    pub name: String,
    pub question: Option<String>,
}

pub struct FormattingSets {
    pub name: String,
    pub fields: Vec<FormatField>,
    pub sets: Vec<FormatSet>,
}

#[derive(Debug)]
pub struct FormatFieldAnswer {
    pub name: String,
    pub answer: String,
}

impl FormatFieldAnswer {
    pub fn placeholder(&self) -> String {
        let mut result = String::new();
        result.push('{');
        result.push_str(&self.name);
        result.push('}');
        result
    }
}

impl FormattingSets {
    pub fn check_answer_names(&self, answers: &Vec<FormatFieldAnswer>) {
        let names: Vec<String> = self.fields.iter().map(|x| x.name.to_owned()).collect();

        for answer in answers {
            if !names.contains(&answer.name) {
                eprintln!(
                    "gorilla: (error) invalid field name: {}. Valid names: {names:?}",
                    answer.name
                );
                panic!()
            }
        }
    }

    // TODO: foloseste un iterator in loc de functia asta jegoasa
    pub fn generate_words(&self, answers: Vec<FormatFieldAnswer>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for fmt_set in &self.sets {
            let mut final_string = String::new();

            for part in &fmt_set.parts {
                let mut p_text = part.text.to_owned();

                for answer in &answers {
                    p_text = p_text.replace(&answer.placeholder(), &answer.answer)
                }

                let mut mutation_result = MutationResult {
                    original_word: p_text.to_owned(),
                    mutated_words: Vec::new(),
                };

                part.mutations.perform(&mut mutation_result, &p_text);
                if !mutation_result.mutated_words.is_empty() {
                    final_string.push_str(&mutation_result.mutated_words[0])
                }
            }

            result.push(final_string)
        }

        result
    }
}

impl FormatSet {
    pub fn new() -> FormatSet {
        FormatSet { parts: Vec::new() }
    }
}
