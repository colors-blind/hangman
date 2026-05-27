pub struct Word {
    pub answer: String,
    pub length: usize,
    pub correct_count: usize,
    pub representation: String,
}

pub trait CheckLetter {
    fn check_for_letter(&mut self, c: char) -> bool;
}

pub trait CheckComplete {
    fn check_complete(&self) -> bool;
}

impl CheckComplete for Word {
    fn check_complete(&self) -> bool {
        self.correct_count == self.length
    }
}

impl CheckLetter for Word {
    fn check_for_letter(&mut self, c: char) -> bool {
        let mut count: usize = 0;
        let mut response = String::with_capacity(self.length);
        let mut index = 0;
        for letter in self.answer.chars() {
            if letter == c {
                count += 1;
                response.push(c);
            }
            else {
                if self.representation.chars().nth(index) != Some('_') {
                    response.push(self.representation.chars().nth(index).unwrap());
                }
                else {
                    response.push('_');
                }
            }
            index += 1;
        }
        self.representation = response;
        self.correct_count += count;
        count > 0
    }
}
