use piston_window::*;
use input::{RenderEvent, UpdateEvent};

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

fn main() {
	let mut ball_speed_x = 0.0;
	let mut ball_speed_y = 0.0;

	let mut serve_direction = 1.0;

    let mut window: PistonWindow = WindowSettings::new("Rusty Pong", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32]).exit_on_esc(true).build().unwrap();

	let mut p1Rect: Rectangle = Rectangle{x: GEN_PADDING, y: GEN_PADDING, width: PLAYER_WIDTH, height: PLAYER_HEIGHT};
	let mut p2Rect: Rectangle = Rectangle{x: WINDOW_WIDTH - PLAYER_WIDTH - GEN_PADDING, y: GEN_PADDING, width: PLAYER_WIDTH, height: PLAYER_HEIGHT};
	let mut ballRect: Rectangle = Rectangle{x: BALL_START_X, y: BALL_START_Y, width: BALL_DIMEN, height: BALL_DIMEN};

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
        });

		ballRect.x += ball_speed_x;
		ballRect.y += ball_speed_y;

		if ballRect.x < 0.0 || ballRect.x + BALL_DIMEN > WINDOW_WIDTH {
			serve_direction = serve_direction * -1.0;
			ballRect.x = BALL_START_X;
			ballRect.y = BALL_START_Y;
			ball_speed_x = 0.0;
			ball_speed_y = 0.0;
		}

		if ballRect.y + BALL_DIMEN >= WINDOW_HEIGHT || ballRect.y <= 0.0 {
			ball_speed_y = ball_speed_y * -1.0;
		}

		//Collision: ball & p1
		if ballRect.x < p1Rect.x + p1Rect.width &&
    		ballRect.x + ballRect.width > p1Rect.x &&
    		ballRect.y < p1Rect.y + p1Rect.height &&
    		ballRect.y + ballRect.height > p1Rect.y {
			println!("Collision");
			ball_speed_x = ball_speed_x * -1.0;
		}	

		//collision: ball & p2
		if ballRect.x < p2Rect.x + p2Rect.width &&
    		ballRect.x + ballRect.width > p2Rect.x &&
    		ballRect.y < p2Rect.y + p2Rect.height &&
    		ballRect.y + ballRect.height > p2Rect.y {
			println!("Collision");
			ball_speed_x = ball_speed_x * -1.0;
		}		

		if let Some(Button::Keyboard(key)) = event.press_args() {
			//Player 1 input
			if key == Key::S {
				println!("Pressed S (Down)");

				if p1Rect.y + p1Rect.height < WINDOW_HEIGHT - GEN_PADDING {
					p1Rect.y = p1Rect.y + PLAYER_SPEED_Y;
				}
			}

			if key == Key::W {
				println!("Pressed W (Up)");

				if p1Rect.y > GEN_PADDING {
					p1Rect.y = p1Rect.y - PLAYER_SPEED_Y;
				}
			}

			//Player 2 input
			if key == Key::Down {
				println!("Pressed Down");

				if p2Rect.y + p2Rect.height < WINDOW_HEIGHT - GEN_PADDING {
					p2Rect.y = p2Rect.y + PLAYER_SPEED_Y;
				}
			}

			if key == Key::Up {
				println!("Pressed Up");

				if p2Rect.y > 10.0 {
					p2Rect.y = p2Rect.y - PLAYER_SPEED_Y;
				}
			}	

			//Ball start
			if key == Key::Space {
				println!("Pressed space");
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

struct Rectangle {
	x: f64,
	y: f64,
	width: f64,
	height: f64,
}