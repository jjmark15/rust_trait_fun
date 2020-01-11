trait Person {
    fn name(&self) -> &String;
}

trait Housemate: Person {
    fn room_number(&self) -> u32;
}

trait Programmer {
    fn fave_language(&self) -> String {
        "Rust".to_owned()
    }
}

trait ProgrammerHousemate: Housemate + Programmer {}

#[allow(dead_code)]
fn programmer_housemate_greeter(housemate: &dyn ProgrammerHousemate) -> String {
    format!(
        "Hi, my name is {}, I live in room {} and I love {}!",
        housemate.name(),
        housemate.room_number(),
        housemate.fave_language()
    )
}

struct GraduateProgrammer {
    name: String,
    favourite_language: String,
}

impl GraduateProgrammer {
    #[allow(dead_code)]
    fn new(name: String, fave_lang: String) -> GraduateProgrammer {
        GraduateProgrammer {
            name,
            favourite_language: fave_lang,
        }
    }
}

impl ProgrammerHousemate for GraduateProgrammer {}

impl Housemate for GraduateProgrammer {
    fn room_number(&self) -> u32 {
        1
    }
}

impl Person for GraduateProgrammer {
    fn name(&self) -> &String {
        &self.name
    }
}

impl Programmer for GraduateProgrammer {
    fn fave_language(&self) -> String {
        (&self.favourite_language).parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::supertraits::{programmer_housemate_greeter, GraduateProgrammer};

    #[test]
    fn test_programmer_housemate_greeter() {
        let josh = GraduateProgrammer::new("Josh".to_string(), "Rust".to_owned());
        assert_eq!(
            "Hi, my name is Josh, I live in room 1 and I love Rust!",
            programmer_housemate_greeter(&josh)
        )
    }
}
