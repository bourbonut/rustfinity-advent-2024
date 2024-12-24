use std::collections::HashMap;

pub struct SantaList {
    // 1. Define the records field
    records: HashMap<String, bool>,
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> Self {
        SantaList { records: HashMap::new() }
    }

    // 3. Implement the add method
    pub fn add(&mut self, child_name: &str, behavior: bool) {
        self.records.insert(child_name.to_string(), behavior);
    }

    // 4. Implement the remove method
    pub fn remove(&mut self, child_name: &str) {
        self.records.remove(&child_name.to_string());
    }

    // 5. Implement the get method
    pub fn get(&self, child_name: &str) -> Option<bool>{
        if let Some(&behavior) = self.records.get(&child_name.to_string()) {
            Some(behavior)
        } else {
            None
        }
    }

    // 6. Implement the count method
    pub fn count(&self) -> (usize, usize) {
        (
            self.records.iter().filter(|(&ref _name, &behavior)| behavior).count(),
            self.records.iter().filter(|(&ref _name, &behavior)| !behavior).count(),
        )
    }

    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, behavior: bool) -> Vec<String>{
        self.records.iter().filter(|(&ref _name, &behv)| behv == behavior).map(|(&ref name, & _behv)| name.clone()).collect()
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
