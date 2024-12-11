use log::debug;

use crate::Day;

#[derive(Debug)]
pub struct Day5 {
    rules: Vec<(u8, u8)>,
    prints: Vec<Vec<u8>>,
}

impl Day5 {
    fn parse_input(input: Vec<String>) -> Self {
        let mut d = Self {
            rules: vec![],
            prints: vec![],
        };

        let mut br = 0;
        for (i, line) in input.clone().into_iter().enumerate() {
            if line.trim().is_empty() {
                br = i + 1;
                break;
            }
            let a = line
                .split('|')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            d.rules.push((a[0], a[1]));
        }

        debug!("Parsed rules {:?}", d.rules);

        for line in &input[br..] {
            if line.trim().is_empty() {
                if d.prints.is_empty() {
                    panic!("Add one to br");
                }
                continue;
            }

            d.prints.push(
                line.split(',')
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
        }
        debug!("Parsed prints {:?}", d.prints);

        d
    }
}

impl Day for Day5 {
    const DAY_NUMBER: u8 = 5;

    fn part1(input: Vec<String>) -> String {
        let input = Self::parse_input(input);
        let mut result = 0;

        let print_job = input.prints.clone();
        let mut valids = vec![];
        let mut breaks = vec![];
        for (i, prints) in print_job.into_iter().enumerate() {
            let mut valid = true;

            let mut printed: Vec<u8> = vec![];
            for print in &prints {
                for rule in input
                    .rules
                    .iter()
                    .filter(|r| &r.0 == print || &r.1 == print)
                {
                    debug!("{print} vs. {rule:?} with history {printed:?}");

                    if print == &rule.0 && printed.contains(&rule.1) {
                        debug!("print {print} breaks {rule:?}");
                        breaks.push(format!("job {} print {print} breaks {rule:?}", i + 1));
                        valid = false;
                        break;
                    } else if !printed.contains(print) {
                        debug!("pushing {print} to printed");
                        printed.push(*print)
                    }
                }
                if !valid {
                    break;
                }
                debug!("");
            }

            if valid {
                result += prints[prints.len() / 2] as u32; // change 1 with midpoint of prints
                debug!("Print job {} valid {prints:?}", i + 1);
                valids.push(i + 1);
            }
        }
        debug!("valid prints {valids:?}");
        debug!("print breaks: {breaks:#?}");

        format!("{result}")
    }

    fn part2(input: Vec<String>) -> String {
        let input = Self::parse_input(input);
        let mut result = 0;

        let print_job = input.prints.clone();
        let mut invalid_prints = vec![];
        let mut invalids = vec![];
        let mut breaks = vec![];
        for (i, prints) in print_job.into_iter().enumerate() {
            let mut valid = true;

            let mut printed: Vec<u8> = vec![];
            for print in &prints {
                for rule in input
                    .rules
                    .iter()
                    .filter(|r| &r.0 == print || &r.1 == print)
                {
                    debug!("{print} vs. {rule:?} with history {printed:?}");

                    if print == &rule.0 && printed.contains(&rule.1) {
                        debug!("print {print} breaks {rule:?}");
                        breaks.push(format!("job {} print {print} breaks {rule:?}", i + 1));
                        valid = false;
                        break;
                    } else if !printed.contains(print) {
                        debug!("pushing {print} to printed");
                        printed.push(*print)
                    }
                }
                if !valid {
                    break;
                }
                debug!("");
            }

            if !valid {
                debug!("Print job {} valid {prints:?}", i + 1);
                invalids.push(i + 1);
                invalid_prints.push(prints);
            }
        }

        debug!("valid prints {invalids:?}");
        debug!("print breaks: {breaks:#?}");

        for print in invalid_prints {
            debug!("print job {print:?}");
            let rules = input
                .rules
                .iter()
                .filter(|r| print.contains(&r.0) && print.contains(&r.1))
                .map(|r| vec![r.0, r.1])
                .collect::<Vec<Vec<u8>>>();
            debug!("{rules:?}");

            let mut final_print = vec![];
            while final_print.len() < print.len() {
                for doc in &print {
                    if final_print.contains(doc) {
                        continue;
                    }
                    if rules
                        .iter()
                        .filter(|r| r[1] == *doc && !final_print.contains(&r[0]))
                        .count()
                        == 0
                    {
                        final_print.push(*doc)
                    }
                }
            }
            result += final_print[final_print.len() / 2] as u32; // change 1 with midpoint of prints
            debug!("final {final_print:?}");
        }

        format!("{result}")
    }
}
