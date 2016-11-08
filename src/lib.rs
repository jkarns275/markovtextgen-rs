extern crate rand;

/// Module contains all methods and sturcts related to creating and using Markov
/// chains to generate text.
pub mod markov {

    #[derive(PartialEq)]
    pub enum LetterCase {
        /// All letters will be made upper case
        Upper,
        /// All letters will be made lower case
        Lower,
        /// All letters will be left alone
        Any,
    }

    /// The structure used to represent a chain state.
    pub struct Markov {
        /// Every way to begin a sentence that has been fed into the struct.
        seeds: Vec<(String, String)>,
        /// Holds every chain that has been fed into the struct.
        chains: Vec<(String, String, Vec<String>)>,
        /// Should input be sanatized of non alpha-numeric characters
        should_strip: bool,
        /// What should be done to input (see LetterCase enum)
        case: LetterCase
    }

    /// Creates a new empty Markov object.
    pub fn new() -> Markov {
        Markov {
            seeds : vec![],
            chains: vec![],
            should_strip: false,
            case: LetterCase::Any
        }
    }

    /// Collects all of the words from a sentence, split by whitespace.
    /// # Arguments
    /// * `sentence` - A string slice containing the sentence to split.
    fn words<'a>(sentence: &'a str) -> Vec<&str> {
        sentence.split_whitespace().collect()
    }

    /// Strips a sentence of all characters that are not alpha-numeric or whitespace
    /// # Arguments
    /// * `sentence` - A string slice containing the sentence to be cleaned
    fn strip<'a>(sentence: &'a str) -> String {
        let mut s: String = String::from("");
        for c in sentence.chars() {
            match c {
                'a' ... 'z' |
                'A' ... 'Z' |
                '0' ... '9' |
                ' ' | '\n' | '\r' => s.push(c),
                _ => {}
            }
        }
        s
    }

    impl Markov {
        /// Generates a string using the data passed into the markov chain
        /// Returns
        /// * `None` - Will return none if the markov chain is empty
        /// * `Some(sentence)` - Returns Some if the markov chain is not empty,
        ///     the sentnence is not gaurenteed to have the specified length
        ///     (Though it will always be less than or equal to it).
        /// # Arguments
        /// * `length` - The maximum length of the sentence
        pub fn generate(&self, length: i32) -> Option<String> {
            use rand::random;

            if self.seeds.len() == 0 {
                return None;
            }

            let ref x = self.seeds[random::<usize>() % self.seeds.len()];
            let mut words = vec![x.0.to_string(), x.1.to_string()];
            for i in 0usize..(length-2) as usize {
                let next_string = self.next(&words[i], &words[i+1usize]);
                match next_string {
                    Some(s) => {
                        words.push(s);
                    },
                    None => {
                        return Some(words.join(" "));
                    }
                };
            }
            return Some(words.join(" "));
        }

        /// Adds a vector of sentences to the MarkovChain
        /// # Arguments
        /// * `sentences` - A vector of string slices to be added
        /// # Examples
        /// ```
        /// let mut markov = markov::new();
        /// let data = vec!["Hello, how are you?", "What are you going to wear tonight?", "What time is it?"];
        /// markov.should_strip = true;
        /// markov.pass(data);
        /// assert!(markov.seeds.contains(&("Hello".to_string(), "how".to_string())));
        /// ```
        pub fn pass<'a>(&mut self, sentences: Vec<&'a str>) {
            for x in sentences {
                self.pass_str(x);
            }

        }

        /// Adds a sentence to the MarkovChain
        /// # Arguments
        /// * `sentence` - A string containing the sentence to be added
        /// # Examples
        /// ```
        /// let mut markov = markov::new();
        /// markov.pass_string("Hello how are you doing today?".to_string());
        /// assert!(markov.generate(5) == "Hello how are you doign today?".to_string());
        /// ```
        pub fn pass_string(&mut self, sentence: String) {
            let mut words: Vec<String> = words(&sentence).iter().map(|x| x.to_string()).collect();
            if self.should_strip {
                for _ in 0..words.len() {
                    let s: String = words.remove(0);
                    words.push(strip(&s));
                }
            }
            if self.case == LetterCase::Lower {
                for _ in 0..words.len() {
                    let s: String = words.remove(0);
                    words.push(s.to_lowercase());
                }
            } else if self.case == LetterCase::Upper{
                for _ in 0..words.len() {
                    let s: String = words.remove(0);
                    words.push(s.to_uppercase());
                }
            }
            self.seeds.push((words[0].to_string(), words[1].to_string()));
            for i in 0..words.len()-2 {
                self.add(&words[i], &words[i + 1], &words[i + 2]);
            }
        }

        /// Adds a sentence to the MarkovChain
        /// # Arguments
        /// * `sentence` - A &str containing the sentence to be added
        /// # Examples
        /// ```
        /// let mut markov = markov::new();
        /// markov.pass_str("Hello how are you doing today?");
        /// assert!(markov.generate(5) == "Hello how are you doing today?".to_string());
        /// ```
        pub fn pass_str<'a>(&mut self, sentence: &'a str) -> bool {
            let mut words: Vec<String> = words(sentence).iter().map(|x| x.to_string()).collect();
            if self.should_strip {
                for _ in 0..words.len() {
                    let s: String = words.remove(0);
                    words.push(strip(&s));
                }
            }
            if self.case == LetterCase::Lower {
                for _ in 0..words.len() {
                    let s: String = words.remove(0);
                    words.push(s.to_lowercase());
                }
            } else if self.case == LetterCase::Upper{
                for _ in 0..words.len() {
                    let s: String = words.remove(0);
                    words.push(s.to_uppercase());
                }
            }

            if words.len() < 2 {
                false
            } else {
                self.seeds.push((words[0].to_string(), words[1].to_string()));
                for i in 0..words.len()-2 {
                    self.add(&words[i], &words[i + 1], &words[i + 2]);
                }
                true
            }
        }

        fn next(&self, s1: &String, s2: &String) -> Option<String> {
            use rand::random;

            for &(ref v1, ref v2, ref v3) in self.chains.iter() {
                if *v1 == *s1 && *v2 == *s2 {
                    let index = random::<usize>() % v3.len();
                    return Some(v3[index].clone())
                }
            }
            None
        }

        fn has_chain(&self, s1: &str, s2: &str) -> bool {
            for &(ref v1, ref v2, _) in self.chains.iter() {
                if v1 == s1 && v2 == s2 {
                    return true
                }
            }
            false
        }

        fn add_to_chain(&mut self, s1: &str, s2: &str, word: &str) {
            for &mut (ref v1, ref v2, ref mut v3) in self.chains.iter_mut() {
                if v1 == s1 && v2 == s2 {
                    v3.push(word.to_string());
                }
            }
        }

        fn add(&mut self, s1: &str, s2: &str, next: &str) {
            if self.has_chain(s1, s2) {
                self.add_to_chain(s1, s2, next);
            } else {
                self.chains.push((s1.to_string(), s2.to_string(), vec![next.to_string()]));
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use markov;

        #[test]
        #[should_panic]
        fn test_empty_markov() {
            let m = markov::new();
            m.generate(10).unwrap();
        }

        #[test]
        fn test_pass_str() {
            let mut m = markov::new();
            m.pass_str("hello how are you");
            assert!(m.generate(100).unwrap() == "hello how are you".to_string());
        }

        #[test]
        fn test_pass_string() {
            let mut m = markov::new();
            m.pass_string("who what when where why who what".to_string());
            assert!(m.generate(10).unwrap() == "who what when where why who what when where why".to_string());
        }

        #[test]
        fn test_pass() {
            let mut m = markov::new();
            m.should_strip = true;
            let data = vec!["Hello, how are you?", "What are you going to wear tonight?", "What time is it?"];
            m.pass(data);
            assert!(m.seeds.contains(&("Hello".to_string(), "how".to_string())));
            m.generate(10).unwrap();
        }

        // Test pass with the 'strip all non alpha-numeric chars' flag
        #[test]
        fn test_pass_strip() {
            let mut m = markov::new();
            m.should_strip = true;
            let sentences = vec!["Hello, how are you?"];
            m.pass(sentences);
            assert!(m.generate(10).unwrap() == "Hello how are you");
        }

        #[test]
        fn test_strip() {
            let m = "Hello".to_string();
            let n = markov::strip("Hello!");
            assert!(n == m);
        }

        #[test]
        fn test_large_data() {
            use std::error::Error;
            use std::fs::File;
            use std::io::prelude::*;
            use std::path::Path;

            let mut m = markov::new();
            m.should_strip = false;
            m.case = markov::LetterCase::Lower;

            let mut file = match File::open(&Path::new("testdata/data.txt")) {
                Err(why) => panic!("couldn't create testdata/data.txt: {}",
                            why.description()),
                Ok(file) => file,
            };

            let mut s = String::new();

            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read testdata/data.txt: {}",
                                    why.description()),
                Ok(_) => (),
            };

            let sentences = s.split("\r").collect::<Vec<&str>>();
            m.pass(sentences);
            for _ in 1..100 {
                println!("Generated \"{}\"", m.generate(100).unwrap());
            }
        }
    }
}
