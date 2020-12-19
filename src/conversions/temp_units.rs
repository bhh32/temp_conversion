pub mod units {
    use std::fmt;
    
    pub enum Units {
        F,
        C,
        K,
        Unknown,
    }

    impl fmt::Display for Units {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Units::C => write!(f, "C"),
                Units::F => write!(f, "F"),
                Units::K => write!(f, "K"),
                Units::Unknown => write!(f, "Unknown"),
            }
        }
    }
}