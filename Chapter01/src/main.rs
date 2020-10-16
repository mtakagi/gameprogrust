mod game;
use game::Game;

fn main()
{
    match Game::new() {
        Ok(mut game) => {
            game.runloop();
        }
        Err(err) => println!("{}", err)
    }
}