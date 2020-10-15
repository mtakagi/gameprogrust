mod game;
use game::Game;

fn main()
{
    match Game::new() {
        Ok(game) => {
            game.runloop();
            game.shutdown();
        }
        Err(err) => println!("{}", err)
    }

}