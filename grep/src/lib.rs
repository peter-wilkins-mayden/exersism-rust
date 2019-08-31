use failure::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Flags {
    print_line_numbers: bool,
    print_file_names: bool,
    case_insensitive: bool,
    invert: bool,
    match_entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut res = Self {
            print_line_numbers: false,
            print_file_names: false,
            case_insensitive: false,
            invert: false,
            match_entire_lines: false,
        };
        for &f in flags {
            match f {
                "-n" => res.print_line_numbers = true,
                "-l" => res.print_file_names = true,
                "-i" => res.case_insensitive = true,
                "-v" => res.invert = true,
                "-x" => res.match_entire_lines = true,
                _ => unreachable!()
            }
        }
        res
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res: Vec<String> = Vec::new();
    for path in files.iter() {
        let mut string = String::new();
        File::open(path)?.read_to_string(&mut string)?;

        let matches: Vec<(usize, &str)> = string.lines()
            .enumerate()
            .filter(|(_, l)| {
                if flags.case_insensitive {
                    let s = l.to_lowercase();
                    let p = pattern.to_lowercase();
                    if flags.match_entire_lines {
                        if flags.invert {
                            s != p
                        } else {
                            s == p
                        }
                    } else {
                        if flags.invert {
                            !s.contains(&p)
                        } else {
                            s.contains(&p)
                        }
                    }
                } else {
                    if flags.match_entire_lines {
                        if flags.invert {
                            **l != *pattern
                        } else {
                            **l == *pattern
                        }
                    } else {
                        if flags.invert {
                            !l.contains(pattern)
                        } else {
                            l.contains(pattern)
                        }
                    }
                }
            }).collect();

        if flags.print_file_names && matches.len() > 0 {
            res.push(path.to_string());
        } else {
            matches.iter().for_each(|(i, l)| {
                if files.len() > 1 && flags.print_line_numbers {
                    res.push(format!("{}:{}:{}", path, i + 1, l))
                } else if files.len() > 1 {
                    res.push(format!("{}:{}", path, l))
                } else if flags.print_line_numbers {
                    res.push(format!("{}:{}", i + 1, l))
                } else {
                    res.push(format!("{}", l))
                }
            });
        }
    }
    Ok(res)
}
