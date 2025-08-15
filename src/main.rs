use std::error::Error;

use poker::*;

fn main() -> Result<(), Box<dyn Error>> {

  let mut game = PokerModel::new();
  game.add_player(HumanView{}, HumanController{});
  game.add_player(ComputerView{}, ComputerController{});

  game.run()?;

  println!("Game successfully exited.");
  Ok(())
}
