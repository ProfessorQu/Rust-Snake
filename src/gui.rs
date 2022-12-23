pub mod gui {
    use raylib::{prelude::*, text::measure_text};

    use crate::{SCREEN_WIDTH, BUTTON_WIDTH, SCREEN_HEIGHT, BUTTON_HEIGHT, BUTTON_FONT_SIZE};

    pub struct Button {
        rec: Rectangle,
        color: Color,
        color_pressed: Color,
        text: String,
        text_color: Color,
        pressed: bool
    }

    impl Button {
        pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, color_pressed: Color, text: String, text_color: Color) -> Self {
            Self {
                rec: Rectangle { x, y, width, height },
                color,
                color_pressed,
                text,
                text_color,
                pressed: false,
            }
        }

        fn check_hover(&self, rl: &RaylibHandle) -> bool {
            let mouse_pos = rl.get_mouse_position();

            mouse_pos.x >= self.rec.x                 && mouse_pos.y >= self.rec.y                  &&
            mouse_pos.x < self.rec.x + self.rec.width && mouse_pos.y < self.rec.y + self.rec.height
        }

        pub fn update(&mut self, rl: &RaylibHandle) -> bool {
            use raylib::consts::MouseButton::*;
            let mut btn_action = false;

            if self.check_hover(rl) {
                if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                    self.pressed = true;
                }
                else if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) && self.pressed {
                    btn_action = true;
                }
                else {
                    self.pressed = false;
                }
            }

            btn_action
        }

        fn draw_text_in_center(&self, d: &mut RaylibDrawHandle) {
            let text_length = measure_text(self.text.as_str(), BUTTON_FONT_SIZE);
            let text_x = self.rec.x as i32 + self.rec.width as i32/ 2 - text_length / 2;
            let text_y = self.rec.y as i32 + self.rec.height as i32/ 2 - BUTTON_FONT_SIZE / 2;
        
            d.draw_text(self.text.as_str(), text_x, text_y, BUTTON_FONT_SIZE, self.text_color);
        }

        pub fn draw(&self, d: &mut RaylibDrawHandle) {
            let color = match self.pressed {
                true => self.color_pressed,
                false => self.color,
            };

            d.draw_rectangle_rec(self.rec, color);
            self.draw_text_in_center(d);
        }
    }

    pub fn mode_menu(rl: &mut RaylibHandle, thread: &RaylibThread) -> String {
        let chosen = false;
    
        let mut self_button = Button::new(
            SCREEN_WIDTH as f32 / 2.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT as f32 / 3.0 - BUTTON_HEIGHT / 2.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            Color::LIME,
            Color::DARKGREEN,
            "Play".to_string(),
            Color::BLACK,
        );
    
        let mut astar_button = Button::new(
            SCREEN_WIDTH as f32 / 4.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT as f32 / 1.5 - BUTTON_HEIGHT / 2.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            Color::LIME,
            Color::DARKGREEN,
            "A*".to_string(),
            Color::BLACK,
        );
    
        let mut ham_button = Button::new(
            3.0 * SCREEN_WIDTH as f32 / 4.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT as f32 / 1.5 - BUTTON_HEIGHT / 2.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            Color::LIME,
            Color::DARKGREEN,
            "Hamiltonian".to_string(),
            Color::BLACK,
        );
    
        while !rl.window_should_close() && !chosen {
            if self_button.update(&rl) {
                return "self".to_string();
            }
            else if astar_button.update(&rl) {
                return "a*".to_string();
            }
            else if ham_button.update(&rl) {
                return "ham".to_string();
            }
    
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::LIGHTGRAY);
    
            self_button.draw(&mut d);
            astar_button.draw(&mut d);
            ham_button.draw(&mut d);
        }
    
        String::new()
    }

    pub fn astar_random_menu(rl: &mut RaylibHandle, thread: &RaylibThread) -> Option<bool> {
        let mut random_button = Button::new(
            SCREEN_WIDTH as f32 / 4.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT as f32 / 1.5 - BUTTON_HEIGHT / 2.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            Color::LIME,
            Color::DARKGREEN,
            "Random dir".to_string(),
            Color::BLACK,
        );
    
        let mut choose_button = Button::new(
            3.0 * SCREEN_WIDTH as f32 / 4.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT as f32 / 1.5 - BUTTON_HEIGHT / 2.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            Color::LIME,
            Color::DARKGREEN,
            "Most space".to_string(),
            Color::BLACK,
        );
    
        while !rl.window_should_close() {
            if random_button.update(&rl) {
                return Some(true)
            }
            else if choose_button.update(&rl) {
                return Some(false)
            }
    
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::LIGHTGRAY);
    
            random_button.draw(&mut d);
            choose_button.draw(&mut d);
        }

        return None
    }
}