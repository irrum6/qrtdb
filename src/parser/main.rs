#[derive(Clone)]
enum States {
    Start,
    X,
    OpenBracket,
    ClosedBracket,
    Invalid,
}
impl States {
    fn cmp_self(&self, s: &States) -> bool {
        let result = match s {
            States::Start => matches!(*self, States::Start),
            States::X => matches!(*self, States::X),
            States::OpenBracket => matches!(*self, States::OpenBracket),
            States::ClosedBracket => matches!(*self, States::ClosedBracket),
            States::Invalid => matches!(*self, States::Invalid),
        };
        return result;
    }
}
struct Automata {
    internal_state: States,
    finite_states: Vec<States>,
    open_bracket_counter: u32,
    closed_bracket_counter: u32,
}
impl Automata {
    pub fn new() -> Automata {
        let finite_states = vec![States::ClosedBracket, States::X];
        return Automata {
            internal_state: States::Start,
            finite_states,
            open_bracket_counter: 0,
            closed_bracket_counter: 0,
        };
    }

    pub fn open_bracket_counter_increase(&mut self) {
        self.open_bracket_counter += 1;
    }
    pub fn closed_bracket_counter_increase(&mut self) {
        self.closed_bracket_counter += 1;
    }
    pub fn reset(&mut self) {
        self.open_bracket_counter = 0;
        self.closed_bracket_counter = 0;
        self.internal_state = States::Start;
    }

    pub fn get_internal_state(&self) -> States {
        return self.internal_state.clone();
    }
    pub fn set_internal_state(&mut self, s: States) {
        self.internal_state = s;
    }

    pub fn transit(&mut self, c: char) {
        if 'x' == c {
            self.char_x_transform();
        }
        if '(' == c {
            self.open_bracket_set();
        }
        if ')' == c {
            self.closed_bracket_set();
        }
    }
    pub fn char_x_transform(&mut self) {
        let s0 = self.get_internal_state();
        let result = match s0 {
            States::Start => States::X,
            States::X => States::X,
            States::OpenBracket => States::OpenBracket,
            States::ClosedBracket => States::Invalid,
            States::Invalid => States::Invalid,
        };
        self.set_internal_state(result);
    }
    pub fn open_bracket_set(&mut self) {
        let s0 = self.get_internal_state();
        self.open_bracket_counter_increase();
        let result = match s0 {
            States::Start => States::OpenBracket,
            States::X => States::Invalid,
            States::ClosedBracket => States::Invalid,
            States::OpenBracket => States::OpenBracket,
            States::Invalid => States::Invalid,
        };
        self.set_internal_state(result);
    }
    pub fn closed_bracket_set(&mut self) {
        let s0 = self.get_internal_state();
        self.closed_bracket_counter_increase();
        let result = match s0 {
            States::Start => States::Invalid,
            States::OpenBracket => States::ClosedBracket,
            States::X => States::Invalid,
            States::ClosedBracket => States::ClosedBracket,
            States::Invalid => States::Invalid,
        };
        self.set_internal_state(result);
    }
    pub fn is_finite_state(&mut self) -> bool {
        let mut is_finite_state = false;
        let bracket_numbers_match = self.open_bracket_counter == self.closed_bracket_counter;
        println!(
            "{},{}",
            self.open_bracket_counter, self.closed_bracket_counter
        );
        for state in &self.finite_states {
            if self.internal_state.cmp_self(state) {
                is_finite_state = true;
                break;
            }
        }
        return is_finite_state && bracket_numbers_match;
    }
    pub fn accept_string(&mut self, s: String) -> bool {
        self.reset();
        for (i, c) in s.chars().enumerate() {
            if 'x' != c && '(' != c && ')' != c {
                println!("{}:{}->bad input", i, c);
                return false;
            }
            self.transit(c);
        }
        if self.is_finite_state() {
            return true;
        }
        return false;
    }
}
#[cfg(test)]
mod tests {
    use super::Automata;
    #[test]
    fn name() {
        let mut das_auto = Automata::new();
        assert_eq!(das_auto.accept_string(String::from("x")), true);
        assert_eq!(das_auto.accept_string(String::from("x(")), false);
        assert_eq!(das_auto.accept_string(String::from("x)")), false);
        assert_eq!(das_auto.accept_string(String::from("(x")), false);
        assert_eq!(das_auto.accept_string(String::from("(x)")), true);
        assert_eq!(das_auto.accept_string(String::from("((x)")), false);
        assert_eq!(das_auto.accept_string(String::from("(x))")), false);
        assert_eq!(das_auto.accept_string(String::from("((x))")), true);
        //this one is not ok because rules x not before (
        assert_eq!(das_auto.accept_string(String::from("(x(x)x)")), false);
    }
}
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut das_auto = Automata::new();
    if args.len() >= 2 {
        let accepted = match das_auto.accept_string(args[1].clone()) {
            true => "input accepted",
            false => "input not accepted",
        };
        println!("{}", accepted);
        return;
    }

    let mut line = String::new();
    let exit = String::from("exit");
    loop{
        println!(">type a word");
        stdin().read_line(&mut line).unwrap();//hmm

        if line.trim().eq(&exit) {
            println!("exitas ");
            break;
        }
        let accepted = match das_auto.accept_string(line.trim().to_string()) {
            true => "input accepted",
            false => "input not accepted",
        };
        println!("{}", accepted);
        line.truncate(0);//oh yeah
    }
}
