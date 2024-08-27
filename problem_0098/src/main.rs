use std::fs;
use std::collections::HashMap;

const ASCII_OFFSET: u8 = 65;
const LETTER_COUNT: usize = 26;

fn main() {
    let all_words = fs::read_to_string("words.txt").unwrap();
    let stripped = all_words.replace("\"", "");
    let words = stripped.split(",").collect::<Vec<&str>>();
    let d = Dictionary::new();

    println!("Answer: {}", d.max_anagramic_square(&words));
}

fn is_prime(number: u8) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as u8;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn generate_prime_numbers(length: usize) -> Vec<u128> {
    let mut i = 0;
    let mut list = vec![];

    loop {
        if is_prime(i) {
            list.push(i as u128);
        }

        if list.len() == length {
            break;
        }

        i += 1;
    }

    list
}

struct Dictionary {
    primes: Vec<u128>
}

impl Dictionary {
	pub fn new() -> Dictionary {
		Dictionary {
            primes: generate_prime_numbers(LETTER_COUNT)
		}
	}

	fn max_anagramic_square(&self, list: &Vec<&str>) -> u128 {
        let mut h: HashMap<u128, Vec<String>> = HashMap::new();
        for word in list {
            let f = self.prime_factor(word);
            h.entry(f)
                .and_modify(|n| n.push(word.to_string()))
                .or_insert(vec![word.to_string()]);
        }
        // Drop off all the non-anagram words
        h.retain(|_key,v| v.len() > 1);

        let mut max = 0;
        for (_k, v) in h {
            let n = self.find_square_value(&v, v[0].len());
            if n > max {
                max = n;
            }
        }

		return max
	}

    fn find_square_value(&self, words: &Vec<String>, max: usize) -> u128 {
        println!("{:?}", words);
        let config = to_config(&words[0]);

        let maps: Vec<Vec<usize>> = words.iter().map(|n|
            n.bytes().map(|byte| config[&byte]).collect()
        ).collect();

        let mut start = 1;
        let mut squares = vec![];
        loop {
            let square: u128 = start * start;
            let len = ((square as f64).log10() + 1.0) as usize;
            if len == max {
                squares.push(square);
            } else if len > max {
                break;
            }
            start += 1;
        }

        let mut max = 0;
        'outer: for s in &squares {
            let n = s.to_string();
            let c = to_config(&n);

            let list: Vec<usize> = n
                .bytes()
                .map(|k| c[&k])
                .collect();

            if list == maps[0] {
                for is in &squares {
                    let m = is.to_string();
                    let mut list = vec![];
                    for k in m.bytes() {
                        if let Some(d) = c.get(&k) {
                            list.push(*d)
                        }
                    }

                    if list == maps[1] {
                        if *s > max {
                            max = *s;
                        }
                        if *is > max {
                            max = *is
                        }
                        println!("{}, {}", s, is);
                        break 'outer;
                    }
                }
            }
        }

        return max
    }


    fn prime_factor(&self, word: &str) -> u128 {
            word
                .bytes()
                .map(|c| self.primes[(c - ASCII_OFFSET) as usize])
                .product()
    }
}

fn to_config(word: &String) -> HashMap<u8, usize> {
    let mut config = HashMap::new();
    for (index, byte) in word.bytes().enumerate() {
        config.insert(byte, index);
    }
    return config
}

#[test]
fn test_max_anagramic_square() {
    let d = Dictionary::new();
    let list = vec!["CARE", "RACE", "HORSE"];
    assert_eq!(d.max_anagramic_square(&list), 9216);
}
