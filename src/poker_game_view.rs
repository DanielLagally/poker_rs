use std::io::{self};

pub trait PokerViewTrait {
  fn get_player_action(&self) -> io::Result<String>;
  fn report(&self, s: &str);
}

pub struct HumanView {}

impl PokerViewTrait for HumanView {
    fn get_player_action(&self) -> Result<String, io::Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn report(&self, s: &str) {
        println!("{s}");
    }
}

pub struct ComputerView {}

impl PokerViewTrait for ComputerView {
    fn get_player_action(&self) -> io::Result<String> {
        todo!()
    }

    fn report(&self, s: &str) { }
}
