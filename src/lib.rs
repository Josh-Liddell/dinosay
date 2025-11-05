pub trait Dinosay {
    fn dinosay(&self);
}

impl Dinosay for str {
    fn dinosay(&self) {
        let lines = get_lines(self);
        let max = lines.iter().map(|line| line.len()).max().unwrap();
        let border = "-".repeat(max + 2) + "\n";
        let border_lines = lines
            .iter()
            .map(|line| {
                let mut spacer = String::new();
                if line.len() < max {
                    spacer = " ".repeat(max - line.len());
                }
                format!("|{line}{spacer}|\n")
            })
            .collect::<Vec<_>>()
            .join("");

        let dino = r#"   \   _
    \ (_ \
        \ \_.----._
         \         \
          |  ) |  ) \__
          |_|--|_|'-.__\"#;

        println!("{border}{border_lines}{border}{dino}");
    }
}

fn get_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_words = Vec::new();
    let mut current_length = 0;

    text.split_whitespace().for_each(|word| {
        let len = if current_words.is_empty() {
            word.len()
        } else {
            current_length + word.len() + 1
        };

        if len <= 41 {
            current_words.push(word);
            current_length = len;
        } else {
            lines.push(current_words.join(" "));
            current_words = vec![word];
            current_length = word.len();
        }
    });

    if !current_words.is_empty() {
        lines.push(current_words.join(" "));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_work() {
        let sample = "Hello there my fellow dinosaurs I have found a new source of fresh leaves by the river please come and join me for a delicious lunch";
        assert_eq!(
            get_lines(sample),
            vec![
                "Hello there my fellow dinosaurs I have",
                "found a new source of fresh leaves by the",
                "river please come and join me for a",
                "delicious lunch"
            ]
        );
    }
}
