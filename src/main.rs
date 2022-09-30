use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window::{self, get_height, get_width};
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 10.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.5;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .high_dpi(true)
        .fullscreen(true)
        .build()?
        .run(GameState::new)
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    winner: String,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./img/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (get_height(ctx) as f32 - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./img/player2.png")?;
        let player2_position = Vec2::new(
            get_width(ctx) as f32 - player2_texture.width() as f32 - 16.0,
            (get_height(ctx) as f32 - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./img/ball.png")?;
        let ball_position = Vec2::new(
            get_width(ctx) as f32 / 2.0 - ball_texture.width() as f32 / 2.0,
            get_height(ctx) as f32 / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            winner: String::new(),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        self.ball.position += self.ball.velocity;

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0
            || self.ball.position.y + self.ball.height() >= get_height(ctx) as f32
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.position.x > WINDOW_WIDTH {
            self.winner = "Player 1".to_string();
        } else if self.ball.position.x < 0.0 {
            self.winner = "Player 2".to_string();
        }

        if !self.winner.is_empty() {
            self.winner
                .push_str(" wins!\nPress Enter to Restart or Esc to quit game");
            let mut winner_text = Text::new(
                self.winner.to_string(),
                Font::vector(ctx, "./fonts/wheaton.otf", 32.0)?,
            );
            let text_position = Vec2::new(
                get_width(ctx) as f32 / 2.0 - 400.0,
                get_height(ctx) as f32 / 2.0 - 100.0,
            );

            winner_text.draw(ctx, text_position);

            if input::is_key_down(ctx, Key::Enter) {
                self.winner = String::new();
                self.ball.position = Vec2::new(
                    get_width(ctx) as f32 / 2.0 - self.ball.texture.width() as f32 / 2.0,
                    get_height(ctx) as f32 / 2.0 - self.ball.texture.height() as f32 / 2.0,
                );
                self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
            }
        }

        Ok(())
    }
}
