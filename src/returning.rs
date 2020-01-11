struct Dragon {}

struct Hydra {}

trait Monster {
    fn noise(&self) -> &'static str;
}

impl Monster for Dragon {
    fn noise(&self) -> &'static str {
        "brwaaagh!"
    }
}

impl Monster for Hydra {
    fn noise(&self) -> &'static str {
        "thyaaaa!"
    }
}

#[allow(dead_code)]
fn random_monster(random_number: f64) -> Box<dyn Monster> {
    if random_number < 0.5 {
        Box::new(Dragon {})
    } else {
        Box::new(Hydra {})
    }
}

#[cfg(test)]
mod tests {
    use crate::returning::random_monster;

    #[test]
    fn test_random_animal() {
        let random_number = 0.234;
        let monster = random_monster(random_number);
        assert_eq!("brwaaagh!", monster.noise(), "assert monster noise")
    }
}
