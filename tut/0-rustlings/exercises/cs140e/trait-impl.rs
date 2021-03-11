// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16),
}

// What traits does `Duration` need to implement?

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        let this = match self {
            Duration::MilliSeconds(x) => *x,
            Duration::Seconds(x) => (*x as u64)*1000,
            Duration::Minutes(x) => (*x as u64)*60*1000
        };
        let other = match other {
            Duration::MilliSeconds(x) => *x,
            Duration::Seconds(x) => (*x as u64)*1000,
            Duration::Minutes(x) => (*x as u64)*60*1000
        };
        this == other
    }
}

#[test]
fn traits() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
