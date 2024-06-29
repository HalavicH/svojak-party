use crate::core::game_pack::game_pack_entites::GamePack;
use crate::core::game_pack::pack_content_entities::{PackContent, Question, Topic};

/// Takes game pack and generates a md table of such format:
/// ## Round: <round>
/// ### Topic: <topic>
/// | Question price | answer |
/// |----------------|--------|
/// | <question>     | <answer>|
/// | <question>     | <answer>|
/// ### Topic: <topic>
/// | Question price | answer |
/// |----------------|--------|
/// | <question>     | <answer>|
/// | <question>     | <answer>|
/// ## Round: <round>
/// ...


trait ToMd {
    fn to_md(&self) -> String;
}

impl ToMd for Question {
    fn to_md(&self) -> String {
        let mut md = String::new();
        md.push_str("| ");
        md.push_str(&self.price.to_string());
        md.push_str(" | ");
        md.push_str(&self.correct_answer.iter().map(|a| a.content.clone()).collect::<Vec<String>>().join(" "));
        md.push_str(" |");
        md
    }
}


impl ToMd for Topic {
    fn to_md(&self) -> String {
        let mut md = String::new();
        md.push_str("### Topic: ");
        md.push_str(&self.name);
        md.push_str("\n");
        md.push_str("| Question price | answer |\n");
        md.push_str("|----------------|--------|\n");
        let mut values: Vec<Question> = self.questions.values().cloned().collect();
        // sort by price
        values.sort_by(|q1, q2| q1.price.cmp(&q2.price));

        for question in values {
            md.push_str(&question.to_md());
            md.push_str("\n");
        }
        md
    }
}

impl ToMd for PackContent {
    fn to_md(&self) -> String {
        let mut md = String::new();
        for round in self.rounds.iter() {
            md.push_str("## Round: ");
            md.push_str(&round.name);
            md.push_str("\n");
            for topic in round.topics.values() {
                md.push_str(&topic.to_md());
                md.push_str("\n");
            }
        }
        md
    }
}

pub fn generate_question_table(game_pack: &PackContent, dst_file_path: String){
    let md = game_pack.to_md();
    std::fs::write(dst_file_path, md).expect("Unable to write file");
}