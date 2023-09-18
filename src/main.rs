use ggez::event::{self, EventHandler};
use ggez::graphics::{Image, Color, Canvas, Mesh, DrawMode, Rect, Text};
use ggez::glam::Vec2;
use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::input::keyboard::KeyCode;

#[derive(PartialEq)]
enum GameStatus {
    Playing,
    Win,
    GameOver,
}

struct Planet {
    planet: Mesh,
    pos_x: f32,
    pos_y: f32,
    size: f32,
}

struct GameState {
    rocket: Image,
    pos_x: f32,
    pos_y: f32,
    vel: f32,
    acl: f32,
    gravity: f32,
    fuel: f32,
    has_fuel: bool,
    power: i32,
    planet: Planet,
    game_status: GameStatus,
}

const FPS: u32 = 60;

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let rocket = Image::from_path(
            ctx,
            "/rocket.png",
        )?;

        let planet = Planet {
            planet: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, 200.0, 50.0),
                Color::GREEN,
            )?,
            pos_x: 100.0,
            pos_y: 750.0,
            size: 200.0
        };

        Ok(GameState {
            rocket,
            pos_x : 175.0,
            pos_y : 0.0,
            vel: 0.0,
            acl: 0.0,
            gravity: 2.5,
            fuel: 100.0,
            has_fuel: true,
            power : 0,
            planet,
            game_status: GameStatus::Playing,
        })
    }

    fn reset_game(&mut self) {
        self.pos_x = 175.0;
        self.pos_y = 0.0;
        self.vel = 0.0;
        self.acl = 0.0;
        self.gravity = 2.5;
        self.fuel = 100.0;
        self.power = 0;
        self.game_status = GameStatus::Playing;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(FPS) && self.game_status == GameStatus::Playing {
            let acl = self.acl / FPS as f32;
            let gravity = self.gravity / FPS as f32;

            self.vel += gravity;

            if self.power == 1 && self.fuel >= 0.1 {
                self.acl = 1.0;
                self.vel -= acl;
                self.fuel -= 0.1;
            } else if self.power == 2 && self.fuel >= 0.3 {
                self.acl = 3.0;
                self.vel -= acl;
                self.fuel -= 0.3;
            } else if self.power == 3 && self.fuel >= 0.6 {
                self.acl = 6.0;
                self.vel -= acl;
                self.fuel -= 0.6;
            }

            self.pos_y += self.vel;

            let rocket_hitbox = Rect::new(
                self.pos_x,
                self.pos_y,
                self.rocket.width() as f32,
                self.rocket.height() as f32,
            );

            let planet_hitbox = Rect::new(
                self.planet.pos_x,
                self.planet.pos_y,
                self.planet.size,
                self.planet.size,
            );

            if self.fuel < 1.0 && self.has_fuel {
                println!("You ran out of fuel!");
                self.has_fuel = false;
                self.power = 0;
            }

            if rocket_hitbox.overlaps(&planet_hitbox) {
                if self.vel <= 2.0 {
                    self.vel = 0.0;
                    println!("Landing successful! You won!");
                    println!("Press R to play again!");
                    self.game_status = GameStatus::Win;
                } else {
                    self.vel = 0.0;
                    println!("Landing too harsh! Game Over");
                    println!("Press R to play again!");
                    self.game_status = GameStatus::GameOver;
                }
                self.gravity = 0.0;
                self.power = 0;
            }     
        }

        Ok(())    
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::CYAN);

        let info_fuel = format!(
            "Fuel: {} l", self.fuel.round()
        );
        canvas.draw(&Text::new(info_fuel), Vec2::new(10.0, 10.0));

        let info_vel = format!(
            "Velocity: {} km/h", self.vel.round()
        );
        canvas.draw(&Text::new(info_vel), Vec2::new(10.0, 30.0));

        let info_power = format!(
            "Throttle Setting: {}", self.power
        );
        canvas.draw(&Text::new(info_power), Vec2::new(10.0, 50.0));

        let info_land = format!(
            "Landing: {} m", ((self.planet.pos_y - 54.0) - self.pos_y).round()
        );
        canvas.draw(&Text::new(info_land), Vec2::new(10.0, 70.0));

        canvas.draw(&self.rocket, Vec2::new(self.pos_x, self.pos_y));

        canvas.draw(
            &self.planet.planet, Vec2::new(self.planet.pos_x, self.planet.pos_y)
        );

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        repeat: bool,
    ) -> GameResult {
        if !repeat {
            match input.keycode {
                Some(KeyCode::Up) => {
                    if self.power < 3 {
                        self.power += 1;
                        println!("Throttling Up!");
                    } else if self.power == 3 {
                        println!("Max Throttle!");
                    }
                }
                Some(KeyCode::Down) => {
                    if self.power > 0 {
                        self.power -= 1;
                        println!("Throttling Down!");   
                    } else if self.power == 0 {
                        println!("Engine Off!");
                    }
                }
                Some(KeyCode::Right) => {
                    self.pos_x += 5.0;
                }
                Some(KeyCode::Left) => {
                    self.pos_x -= 5.0;
                }
                Some(KeyCode::R) => {
                    GameState::reset_game(self);
                }
                Some(KeyCode::Escape) => {
                    _ctx.request_quit()
                }
                _ => (),
            }
        }

        Ok(())
    }
}

fn main() {
    let cb = ContextBuilder::new("rusty-rocketeer", "augusto-molina")
        .window_setup(conf::WindowSetup::default().title("Rusty Rocketeer!"))
        .window_mode(conf::WindowMode::default()
            .dimensions(400.0, 800.0)
            .resizable(false)
        );

    if let Err(e) = run_game(cb) {
        println!("Error: {}", e);
    }
}

fn run_game(cb: ContextBuilder) -> GameResult {
    let (mut ctx, event_loop) = cb.build()?;
    let game = GameState::new(&mut ctx)?;
    println!("Up and Down for throttle");
    println!("Left and Right for movement");
    println!("R to reset");
    println!("Esc to exit");
    event::run(ctx, event_loop, game);
}