use std::error::Error;

use crate::{PokerControllerTrait, PokerViewTrait, UserAction, UserActionError};

struct PokerCard {
    value: i32,
    colour: PokerCardColour,
}

enum PokerCardColour {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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
}

pub trait PokerModelTrait {
    fn new() -> Self;
    fn add_player<View: PokerViewTrait + 'static, Controller: PokerControllerTrait + 'static>(
        &mut self,
        view: View,
        controller: Controller,
    );
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl PokerModelTrait for PokerModel {
    fn new() -> Self {
        PokerModel {
            pot: 0,
            river: Vec::new(),
            players: Vec::new(),
        }
    }

    fn add_player<View: PokerViewTrait + 'static, Controller: PokerControllerTrait + 'static> (&mut self, view: View, controller: Controller) {
        self.players.push(Player {
            view: Box::new(view),
            controller: Box::new(controller),
            chips: 0,
            hand: [None, None],
        });
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        self.players.iter().for_each(|p| p.view.report("Welcome to poker."));

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
