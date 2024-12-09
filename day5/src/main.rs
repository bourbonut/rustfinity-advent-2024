pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1
        let mut parts = csv_row.split(",");
        let name = parts.next().ok_or("No name")?;
        let good_deeds = match parts.next().ok_or("No good_deeds")?.parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Err("Not a number"),
        };
        let bad_deeds = match parts.next().ok_or("No bad_deeds")?.parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Err("Not a number"),
        };

        Ok(Self::new(name.to_string(), good_deeds, bad_deeds))
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}


fn main() {
    let output1 = match Kid::parse_row("Alice,12,10") {
        Ok(val) => val.name,
        Err(_) => String::from("Error 1"),
    };
    let output2 = match Kid::parse_row("Bob,,10") {
        Ok(val) => val.name,
        Err(_) => String::from("Error 2"),
    };
    assert_eq!(output1, String::from("Alice"));
    assert_eq!(output2, String::from("Error 2"));
}
