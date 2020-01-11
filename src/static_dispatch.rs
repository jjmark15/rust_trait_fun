trait Developer {
    fn think(&self) -> &str;

    fn solve(&self) -> String {
        format!("{}, this isn't right", &self.think())
    }
}

trait BackendDeveloper: Developer {}

trait FrontendDeveloper: Developer {}

struct JavaDeveloper {}

impl BackendDeveloper for JavaDeveloper {}

impl Developer for JavaDeveloper {
    fn think(&self) -> &str {
        "hmmmmm"
    }
}

struct JavaScriptDeveloper {}

impl FrontendDeveloper for JavaScriptDeveloper {}

impl Developer for JavaScriptDeveloper {
    fn think(&self) -> &str {
        "ummmmm"
    }
}

#[allow(dead_code)]
fn test_developer<GenericDev: Developer>(dev: &GenericDev) -> String {
    dev.solve()
}

#[cfg(test)]
mod tests {
    use crate::static_dispatch::{test_developer, Developer, JavaDeveloper, JavaScriptDeveloper};

    #[test]
    fn test_solve_java_developer() {
        let dev: JavaDeveloper = JavaDeveloper {};
        assert_eq!("hmmmmm, this isn't right", dev.solve())
    }

    #[test]
    fn test_test_developer() {
        let dev: JavaScriptDeveloper = JavaScriptDeveloper {};
        assert_eq!("ummmmm, this isn't right", test_developer(&dev))
    }
}
