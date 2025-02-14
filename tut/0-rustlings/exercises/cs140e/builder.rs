// FIXME: Make me pass! Diff budget: 30 lines.

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string(&mut self, s: &str) -> &mut Self {
        self.string = Some(String::from(s));
        self
    }

    fn number(&mut self, n: usize) -> &mut Self {
        self.number = Some(n);
        self
    }
}

impl ToString for Builder {
    // Implement the trait
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut n = String::new();
        if let Some(string) = &self.string {
            s = string.to_string();
        }
        if let Some(number) = &self.number {
            n = number.to_string();
        }
        if s.len() > 0 && n.len() > 0 {
            s + " " + &n
        } else {
            s + &n
        }
    }
}

// Do not modify this function.
#[test]
fn builder() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default().string(&"heap!".to_owned()).to_string();

    assert_eq!(c, "heap!");
}
