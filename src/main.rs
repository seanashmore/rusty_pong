use piston_window::*;
use input::{RenderEvent, UpdateEvent};
extern crate find_folder;

static WINDOW_HEIGHT: f64 = 480.0;
static WINDOW_WIDTH: f64 = 640.0;

static PLAYER_HEIGHT: f64 = 100.0;
static PLAYER_WIDTH: f64 = 20.0;

static BALL_DIMEN: f64 = 25.0;
static BALL_SPEED: f64 = 0.6;

static PLAYER_SPEED_Y: f64 = 40.0;

static BALL_START_X: f64 = WINDOW_WIDTH / 2.0 - (BALL_DIMEN / 2.0);
static BALL_START_Y: f64 = WINDOW_HEIGHT / 2.0 - (BALL_DIMEN / 2.0);

static GEN_PADDING: f64 = 10.0;

static TEXT_P1_SCORE_X: f64 = (WINDOW_WIDTH / 4.0);
static TEXT_P2_SCORE_X: f64 = (WINDOW_WIDTH / 4.0) * 3.0;
static TEXT_PADDING_Y: f64 = 20.0;
static TEXT_SIZE: u32 = 12;

fn main() {
	let mut ball_speed_x = 0.0;
	let mut ball_speed_y = 0.0;
	let mut serve_direction = 1.0;

    let mut window: PistonWindow = WindowSettings::new("Rusty Pong", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32]).exit_on_esc(true).build().unwrap();

	let mut p1Rect: Rectangle = Rectangle{x: GEN_PADDING, y: GEN_PADDING, width: PLAYER_WIDTH, height: PLAYER_HEIGHT};
	let mut p2Rect: Rectangle = Rectangle{x: WINDOW_WIDTH - PLAYER_WIDTH - GEN_PADDING, y: GEN_PADDING, width: PLAYER_WIDTH, height: PLAYER_HEIGHT};
	let mut ballRect: Rectangle = Rectangle{x: BALL_START_X, y: BALL_START_Y, width: BALL_DIMEN, height: BALL_DIMEN};

	let mut p1Score = 0;
	let mut p2Score = 0;

	let assets = find_folder::Search::ParentsThenKids(3,3)
	.for_folder("assets").unwrap();
	let ref font = assets.join("8bit.ttf");
	let factory = window.factory.clone();
	let texture_settings = TextureSettings::new();
	let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.0; 4], graphics);

			//Player 1 (lhs)
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [p1Rect.x, p1Rect.y, p1Rect.width, p1Rect.height],
                      context.transform,
                      graphics);

			//Player 2 (rhs)
            rectangle([0.0, 0.0, 1.0, 1.0], // blue
                      [p2Rect.x, p2Rect.y, p2Rect.width, p2Rect.height],
                      context.transform,
                      graphics);		

			rectangle([0.0, 1.0, 0.0, 1.0], //Green
					  [ballRect.x, ballRect.y, ballRect.width, ballRect.height],
					  context.transform,
					  graphics); 	

			let transform = context.transform.trans(TEXT_P1_SCORE_X, TEXT_PADDING_Y);
			text::Text::new_color([1.0, 1.0, 1.0, 1.0], TEXT_SIZE).draw(
				&p1Score.to_string(),
				&mut glyphs,
				&context.draw_state,
				transform, graphics
			);

			let transform = context.transform.trans(TEXT_P2_SCORE_X, TEXT_PADDING_Y);
			text::Text::new_color([1.0, 1.0, 1.0, 1.0], TEXT_SIZE).draw(
				&p2Score.to_string(),
				&mut glyphs,
				&context.draw_state,
				transform, graphics
			);			 		  
        });

		ballRect.x += ball_speed_x;
		ballRect.y += ball_speed_y;

		if ballRect.x < 0.0 || ballRect.x + BALL_DIMEN > WINDOW_WIDTH {
			serve_direction = reverse(serve_direction);

			if serve_direction < 0.0 {
				p1Score = p1Score + 1;
			} else {
				p2Score = p2Score + 1;
			}

			ballRect.x = BALL_START_X;
			ballRect.y = BALL_START_Y;
			ball_speed_x = 0.0;
			ball_speed_y = 0.0;
		}

		if ballRect.y + BALL_DIMEN >= WINDOW_HEIGHT || ballRect.y <= 0.0 {
			ball_speed_y = reverse(ball_speed_y);
		}

		if hasCollided(&ballRect, &p1Rect) {
			ball_speed_x = reverse(ball_speed_x);		
		}

		if hasCollided(&ballRect, &p2Rect) {
			ball_speed_x = reverse(ball_speed_x);	
		}

		if let Some(Button::Keyboard(key)) = event.press_args() {
			//Player 1 input
			if key == Key::S {
				if p1Rect.y + p1Rect.height < WINDOW_HEIGHT - GEN_PADDING {
					p1Rect.y = p1Rect.y + PLAYER_SPEED_Y;
				}
			}

			if key == Key::W {
				if p1Rect.y > GEN_PADDING {
					p1Rect.y = p1Rect.y - PLAYER_SPEED_Y;
				}
			}

			//Player 2 input
			if key == Key::Down {
				if p2Rect.y + p2Rect.height < WINDOW_HEIGHT - GEN_PADDING {
					p2Rect.y = p2Rect.y + PLAYER_SPEED_Y;
				}
			}

			if key == Key::Up {
				if p2Rect.y > GEN_PADDING {
					p2Rect.y = p2Rect.y - PLAYER_SPEED_Y;
				}
			}	

			//Ball start
			if key == Key::Space {
				if ball_speed_x == 0.0 {
					if serve_direction > 0.0 {
						ball_speed_x = BALL_SPEED;
					} else {
						ball_speed_x = -BALL_SPEED;
					}
					ball_speed_y = BALL_SPEED;
				}
			}		
		}
    }
}

fn hasCollided(rect1: &Rectangle, rect2: &Rectangle) -> bool {
	return rect1.x < rect2.x + rect2.width &&
		rect1.x + rect1.width > rect2.x &&
		rect1.y < rect2.y + rect2.height &&
		rect1.y + rect1.height > rect2.y; 
}

fn reverse(speed: f64) -> f64 {
	return speed * -1.0;
}

struct Rectangle {
	x: f64,
	y: f64,
	width: f64,
	height: f64,
}