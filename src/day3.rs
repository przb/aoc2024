#[inline]
fn parse_digit(byte: u8, digit: &mut [u8], digit_len: &mut usize) -> bool {
    let is_digit = (b'0'..=b'9').contains(&byte);
    if is_digit && *digit_len < digit.len() {
        unsafe {
            *digit.get_unchecked_mut(*digit_len) = byte - b'0';
        }
        *digit_len += 1;
        true
    } else {
        false
    }
}

#[inline]
fn read_memory(input: &str, do_do: bool) -> usize {
    let mut x = [0, 0, 0];
    let mut y = [0, 0, 0];
    let mut x_len = 0;
    let mut y_len = 0;
    let mut is_parsing = false;
    let mut found_comma = false;
    let mut prev = 0;
    let mut sum = 0;

    let mut mul_enabled = true;
    let mut reading_dos = false;

    input.bytes().for_each(|byte| {
        if is_parsing {
            if byte == b')' {
                if y_len > 0 {
                    // x and y are in reverse byte order
                    unsafe {
                        let x = match x_len {
                            1 => *x.get_unchecked(0) as usize,
                            2 => (*x.get_unchecked(0) as usize * 10) + *x.get_unchecked(1) as usize,
                            3 => {
                                (*x.get_unchecked(0) as usize * 100)
                                    + (*x.get_unchecked(1) as usize * 10)
                                    + *x.get_unchecked(2) as usize
                            }
                            _ => panic!("Invalid X Len"),
                        };
                        let y = match y_len {
                            1 => *y.get_unchecked(0) as usize,
                            2 => (*y.get_unchecked(0) as usize * 10) + *y.get_unchecked(1) as usize,
                            3 => {
                                (*y.get_unchecked(0) as usize * 100)
                                    + (*y.get_unchecked(1) as usize * 10)
                                    + *y.get_unchecked(2) as usize
                            }
                            _ => panic!("Invalid Y Len"),
                        };
                        sum += if do_do && mul_enabled {
                            x * y
                        } else if do_do && !mul_enabled {
                            0
                        } else {
                            x * y
                        };
                    }

                    // reset everything
                    x_len = 0;
                    y_len = 0;
                    //is_parsing = false;
                    found_comma = false;
                    prev = 0;
                }
            }
            is_parsing = match prev {
                b'm' => byte == b'u',
                b'u' => byte == b'l',
                b'l' => byte == b'(',
                b'(' => {
                    found_comma = false;
                    parse_digit(byte, &mut x, &mut x_len)
                }
                b'0'..=b'9' => {
                    let is_digit = if found_comma {
                        parse_digit(byte, &mut y, &mut y_len)
                    } else {
                        parse_digit(byte, &mut x, &mut x_len)
                    };

                    is_digit || byte == b','
                }
                b',' => {
                    found_comma = true;
                    parse_digit(byte, &mut y, &mut y_len)
                }
                b')' => false,
                _ => false,
            }
        } else {
            match prev {
                b'd' => {
                    if byte == b'o' {
                        // then we need to decide if its a do or a don't
                        reading_dos = true;
                    } else {
                        reading_dos = false;
                    }
                }
                b'o' => {
                    if byte != b'n' && reading_dos {
                        reading_dos = false;
                        mul_enabled = true;
                    }
                }
                b'n' => {
                    if byte != b'\'' && reading_dos {
                        reading_dos = false;
                        mul_enabled = true;
                    }
                }
                b'\'' => {
                    mul_enabled = if byte == b't' && reading_dos {
                        false
                    } else if reading_dos {
                        true
                    } else {
                        mul_enabled
                    };
                    //mul_enabled = !(byte == b't' && reading_dos);
                    reading_dos = false;
                }
                b't' => {} // dont need to check t, leaving this here so i dont forget tho
                _ => {}    // do nothing
            }
            is_parsing = byte == b'm';
            y_len = 0;
            x_len = 0;
        }
        //println!(
        //    "byte: {}, parsing dos: {}, mul_en: {}",
        //    byte as char, reading_dos, mul_enabled
        //);
        prev = byte;
    });
    sum.into()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    read_memory(input, false)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    read_memory(input, true)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    const SAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    fn get_input() -> String {
        let input_path = "input/2024/day3.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT), 161)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 170778545)
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT_2), 48)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 82868252)
    }
}
