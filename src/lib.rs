use std::fmt::Write;

pub trait Dinosay {
    fn dinosay(&self); //should I just make it a  default impl here?
}

impl Dinosay for str {
    fn dinosay(&self) {
        let lines = get_lines(self);
        let max = lines.iter().map(|line| line.len()).max().unwrap();
        let border = "-".repeat(max + 2) + "\n";
        let border_lines = lines
            .iter()
            .map(|line| {
                let mut spacer = String::new(); // can this be changed?
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

// maybe rewrite this to get it to work a different way and use iterator
fn get_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
        if line.len() == 0 {
            write!(&mut line, "{word}").unwrap();
            continue;
        }
        if line.len() + (word.len() + 1) <= 41 {
            // append the word to the line
            write!(&mut line, " {word}").unwrap();
        } else {
            // push the line and reset the line, with the word
            lines.push(line);
            line = word.to_string();
        }
    }

    if !line.is_empty() {
        lines.push(line);
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
