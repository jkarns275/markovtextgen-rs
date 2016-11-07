extern crate rand;

pub mod markov {
    pub struct Markov {
        seeds: Vec<(String, String)>,
        chains: Vec<(String, String, Vec<String>)>,
    }

    pub fn new() -> Markov {
        Markov {
            seeds : vec![],
            chains: vec![],
        }
    }

    fn words<'a>(sentence: &'a str) -> Vec<&str> {
        sentence.split_whitespace().collect()
    }

    fn strip<'a>(sentence: &'a str) -> String {
        let mut v: Vec<char> = vec![];
        for c in sentence.chars() {
            match c {
                'a' ... 'z' |
                'A' ... 'Z' |
                '0' ... '9' |
                ' ' | '\n' | '\r' => v.push(c),
                _ => {}
            }
        }
        let r = v.into_iter().collect();
        println!("\nDEBUG_ : {}", r);
        r
    }

    impl Markov {
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

        pub fn pass<'a>(&mut self, sentences: Vec<&'a str>, should_strip: bool)
        {
            println!("{}", sentences.len());
            if should_strip {
                for x in sentences {
                    self.pass_string(strip(x))
                }
            } else {
                for x in sentences {
                    println!("{}", x);
                    self.pass_str(x);
                }
            }
        }

        pub fn pass_string(&mut self, sentence: String) {
            let words = words(&sentence);
            if words.len() < 2 {
                return
            }
            self.seeds.push((words[0].to_string(), words[1].to_string()));
            for i in 0..words.len()-2 {
                self.add(words[i], words[i + 1], words[i + 2]);
            }
        }

        pub fn pass_str<'a>(&mut self, sentence: &'a str) {
            println!("{}", sentence);
            let words: Vec<&str> = words(sentence);
            if words.len() < 2 {
                return
            }
            self.seeds.push((words[0].to_string(), words[1].to_string()));
            for i in 0..words.len()-2 {
                self.add(words[i], words[i + 1], words[i + 2]);
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
            let sentences = vec!["Hello how are you", "What time is it", "Hey, is it time to leave?"];
            m.pass(sentences, false);
            m.generate(10).unwrap();
        }

        // Test pass with the 'strip all non alpha-numeric chars' flag
        #[test]
        fn test_pass_strip() {
            let mut m = markov::new();
            let sentences = vec!["Hello, how are you?"];
            m.pass(sentences, true);
            assert!(m.generate(10).unwrap() == "Hello how are you");
        }

        #[test]
        fn test_strip() {
            let m = "Hello".to_string();
            let n = markov::strip("Hello!");
            println!("\nn = [{}]", n);
            assert!(n == m);
        }
    }
}
