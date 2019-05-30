use piston_window::*;
use input::{RenderEvent, UpdateEvent};

static WINDOW_HEIGHT: f64 = 480.0;
static WINDOW_WIDTH: f64 = 640.0;

static PLAYER_HEIGHT: f64 = 100.0;
static PLAYER_WIDTH: f64 = 40.0;

static BALL_DIMEN: f64 = 25.0;
static BALL_SPEED: f64 = 1.0;

static PLAYER_SPEED_Y: f64 = 20.0;

static BALL_START_X: f64 = WINDOW_WIDTH / 2.0 - (BALL_DIMEN / 2.0);
static BALL_START_Y: f64 = WINDOW_HEIGHT / 2.0 - (BALL_DIMEN / 2.0);

static GEN_PADDING: f64 = 10.0;

static P1_X: f64 = GEN_PADDING;
static P2_X: f64 = WINDOW_WIDTH - PLAYER_WIDTH - GEN_PADDING;


fn main() {
	let mut ball_x = BALL_START_X;
	let mut ball_y = BALL_START_Y;
	let mut ball_speed_x = 0.0;
	let mut ball_speed_y = 0.0;

	let mut p1y = GEN_PADDING;
	let mut p2y = GEN_PADDING;

	let mut serve_direction = 1.0;

    let mut window: PistonWindow = WindowSettings::new("Rusty Pong", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

			//Player 1 (lhs)
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [P1_X, p1y, PLAYER_WIDTH, PLAYER_HEIGHT],
                      context.transform,
                      graphics);

			//Player 1 (lhs)
            rectangle([0.0, 0.0, 1.0, 1.0], // blue
                      [P2_X, p2y, PLAYER_WIDTH, PLAYER_HEIGHT],
                      context.transform,
                      graphics);		

			rectangle([0.0, 1.0, 0.0, 1.0], //Green
					  [ball_x, ball_y, BALL_DIMEN, BALL_DIMEN],
					  context.transform,
					  graphics); 	 		  
        });

		ball_x += ball_speed_x;
		ball_y += ball_speed_y;

		if ball_x < 0.0 || ball_x + BALL_DIMEN > 640.0 {
			serve_direction = serve_direction * -1.0;
			ball_x = BALL_START_X;
			ball_y = BALL_START_Y;
			ball_speed_x = 0.0;
			ball_speed_y = 0.0;
		}

		if ball_y + BALL_DIMEN >= 480.0 || ball_y <= 0.0 {
			ball_speed_y = ball_speed_y * -1.0;
		}

		if let Some(Button::Keyboard(key)) = event.press_args() {
			//Player 1 input
			if key == Key::S {
				println!("Pressed S (Down)");

				if p1y + PLAYER_HEIGHT < 480.0 - 10.0 {
					p1y = p1y + PLAYER_SPEED_Y;
				}
			}

			if key == Key::W {
				println!("Pressed W (Up)");

				if p1y > 10.0 {
					p1y = p1y - PLAYER_SPEED_Y;
				}
			}

			//Player 2 input
			if key == Key::Down {
				println!("Pressed Down");

				if p2y + PLAYER_HEIGHT < 480.0 - 10.0 {
					p2y = p2y + PLAYER_SPEED_Y;
				}
			}

			if key == Key::Up {
				println!("Pressed Up");

				if p2y > 10.0 {
					p2y = p2y - PLAYER_SPEED_Y;
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

struct Point {
	x: f64,
	y: f64,
}

struct Rectangle {
	tl: Point,
	tr: Point,
	bl: Point,
	br: Point,
}