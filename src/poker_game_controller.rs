use std::result::Result;

use rand::{random, rngs::ThreadRng, Rng, RngCore};

pub trait PokerControllerTrait {
        fn get_action(&self) -> Result<UserAction, UserActionError>;
}

pub enum UserAction {
    Exit,
    Fold,
    Raise(i64),
    Call,
}

pub enum UserActionError {
    InvalidRaise,
    UnknownArgument,
}

pub struct HumanController {}

impl HumanController {
    pub fn parse_input(&self, input: &str) -> Result<UserAction, UserActionError> {
        // TODO: partial matching, i.e. "e" or "ex" matches "exit"
        let args: Vec<&str> = input.trim_end_matches("\n").split(" ").collect();
        match args.first().copied().unwrap_or("").to_lowercase().as_str() {
            "fold" => Ok(UserAction::Fold),
            "raise" => {
                let amount = args
                    .get(1)
                    .copied()
                    .ok_or(UserActionError::InvalidRaise)?
                    .parse()
                    .map_err(|_| UserActionError::InvalidRaise)?;
                Ok(UserAction::Raise(amount))
            }
            "call" => Ok(UserAction::Call),
            "exit" => Ok(UserAction::Exit),
            _ => Err(UserActionError::UnknownArgument),
        }
    }
}

impl PokerControllerTrait for HumanController {
    fn get_action(&self) -> Result<UserAction, UserActionError> {
        let mut input = String::new();
        loop {
            if std::io::stdin().read_line(&mut input).is_ok() {
                break;
            }
        }
        self.parse_input(&input)
    }
}

pub struct ComputerController {}

impl PokerControllerTrait for ComputerController {
    fn get_action(&self) -> Result<UserAction, UserActionError> {
        match rand::rng().random_range(0..4) {
            0 => Ok(UserAction::Exit),
            1 => Ok(UserAction::Fold),
            2 => Ok(UserAction::Raise(rand::rng().random())),
            3 => Ok(UserAction::Call),
            _ => panic!(),
        }
    }
}
