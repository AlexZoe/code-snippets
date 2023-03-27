pub struct OverlapFinder<'a> {
    input: std::str::Lines<'a>,
}

impl<'a> OverlapFinder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.lines(),
        }
    }
}

struct Section {
    start: u64,
    end: u64,
}

struct SectionPair {
    first: Section,
    second: Section,
}

pub trait Overlap {
    fn full_overlap(&mut self) -> u64;
    fn partial_overlap(&mut self) -> u64;
}

fn extract_section_bounds_from(input: &str) -> SectionPair {
    let sections: Vec<&str> = input.split(',').collect();

    let lhs: Vec<&str> = sections[0].split('-').collect();
    let rhs: Vec<&str> = sections[1].split('-').collect();

    SectionPair {
        first: Section {
            start: lhs[0].parse::<u64>().unwrap(),
            end: lhs[1].parse::<u64>().unwrap(),
        },
        second: Section {
            start: rhs[0].parse::<u64>().unwrap(),
            end: rhs[1].parse::<u64>().unwrap(),
        },
    }
}

impl<'a> Overlap for OverlapFinder<'a> {
    fn full_overlap(&mut self) -> u64 {
        let mut sum = 0;
        while let Some(line) = self.input.next() {
            let sections = extract_section_bounds_from(line);
            if sections.first.start <= sections.second.start
                && sections.first.end >= sections.second.end
            {
                sum += 1;
            } else if sections.second.start <= sections.first.start
                && sections.second.end >= sections.first.end
            {
                sum += 1;
            } else {
            }
        }
        sum
    }

    fn partial_overlap(&mut self) -> u64 {
        let mut sum = 0;
        while let Some(line) = self.input.next() {
            let sections = extract_section_bounds_from(line);
            if sections.first.start <= sections.second.start
                && sections.first.end >= sections.second.start
            {
                sum += 1;
            } else if sections.second.start <= sections.first.start
                && sections.second.end >= sections.first.start
            {
                sum += 1;
            } else {
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::super::file;
    use super::*;

    #[test]
    fn return_one_if_section_contains_other() {
        assert_eq!(1, OverlapFinder::new("2-8,3-7").full_overlap());
    }

    #[test]
    fn return_zero_if_section_does_not_contains_other() {
        assert_eq!(0, OverlapFinder::new("2-3,4-5").full_overlap());
    }

    #[test]
    fn return_full_overlapping_sections() {
        let string = file::get_string_from_file("assets/day4_sample.txt");
        assert_eq!(2, OverlapFinder::new(&string).full_overlap());
    }

    #[test]
    fn return_partial_overlapping_sections() {
        let string = file::get_string_from_file("assets/day4_sample.txt");
        assert_eq!(4, OverlapFinder::new(&string).partial_overlap());
    }
}
