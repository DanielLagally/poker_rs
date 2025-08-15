use std::error::Error;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{PokerControllerTrait, PokerViewTrait, UserAction, UserActionError};

#[derive(Debug)]
struct PokerCard {
    value: CardValue,
    colour: CardColour,
}

#[derive(EnumIter, Clone, Copy, Debug)]
enum CardColour {
    Clubs, Diamonds, Hearts, Spades, }

#[derive(EnumIter, Clone, Copy, Debug)]
enum CardValue {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

pub struct Player {
    view: Box<dyn PokerViewTrait>,
    controller: Box<dyn PokerControllerTrait>,
    chips: i64,
    hand: [Option<PokerCard>; 2],
}

pub struct PokerModel {
    pot: i64,
    river: Vec<PokerCard>,
    players: Vec<Player>,
    deck: Vec<PokerCard>,
}

impl PokerModel {
    fn init_game(&mut self) {
        self.deck = CardColour::iter()
            .flat_map(|colour| CardValue::iter().map(move |value| PokerCard { colour, value }))
            .collect();
    }
}

pub trait PokerModelTrait {
    fn new() -> Self;
    fn add_player<View: PokerViewTrait + 'static, Controller: PokerControllerTrait + 'static>(
        &mut self,
        view: View,
        controller: Controller,
    );
    fn run(&mut self) -> Result<(), Box<dyn Error>>;
}

impl PokerModelTrait for PokerModel {
    fn new() -> Self {
        PokerModel {
            pot: 0,
            river: Vec::new(),
            players: Vec::new(),
            deck: Vec::new(),
        }
    }

    fn add_player<View: PokerViewTrait + 'static, Controller: PokerControllerTrait + 'static>(
        &mut self,
        view: View,
        controller: Controller,
    ) {
        self.players.push(Player {
            view: Box::new(view),
            controller: Box::new(controller),
            chips: 0,
            hand: [None, None],
        });
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.players
            .iter()
            .for_each(|p| p.view.report("Welcome to poker."));

        &self.init_game();

        for i in (0..self.players.len()).cycle() {
            let player = self.players.get(i).unwrap();
            let view = player.view.as_ref();
            let controller = player.controller.as_ref();
            player.view.report("It's your turn.");
            match controller.get_action() {
                Ok(UserAction::Exit) => break,
                Ok(UserAction::Fold) => println!("fold"),
                Ok(UserAction::Raise(amount)) => println!("raise: {amount}"),
                Ok(UserAction::Call) => println!("call"),
                Err(UserActionError::InvalidRaise) => view.report("invalid raise."),
                Err(UserActionError::UnknownArgument) => view.report("invalid argument."),
            }
        }

        Ok(())
    }
}
