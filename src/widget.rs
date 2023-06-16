use std::f32::consts::PI;
use egui::Pos2;
use egui::Rect;
use egui::Stroke;
use egui::Vec2;
use egui::pos2;
use egui::Color32;
use egui_extras::RetainedImage;

pub struct WheelWidget {
    wheel_image: RetainedImage,
    images: Vec<RetainedImage>,
    positions: Vec<Pos2>,
    selected: i32,
}

impl Default for WheelWidget {
    fn default() -> Self {
        let wheel_image = RetainedImage::from_image_bytes("Wheel",
                include_bytes!("../assets/EmoteWheel.png")).unwrap();

        let mut images: Vec<RetainedImage> = Vec::new();

        //1
        images.push(RetainedImage::from_image_bytes("DefaultDance1", 
                include_bytes!("../assets/2BDefaultDance.png")).unwrap());
        
        //2
        images.push(RetainedImage::from_image_bytes("OrangeJustice2", 
                include_bytes!("../assets/2BOrangeJustice.png")).unwrap());

        //3
        images.push(RetainedImage::from_image_bytes("ElectroShuffle3", 
                include_bytes!("../assets/2BElectroShuffle.png")).unwrap());

        //4
        images.push(RetainedImage::from_image_bytes("BreakDance4", 
                include_bytes!("../assets/2BBreakDance.png")).unwrap());

        //5
        images.push(RetainedImage::from_image_bytes("BestMates5", 
                include_bytes!("../assets/2BBestMates.png")).unwrap());
        
        //6
        images.push(RetainedImage::from_image_bytes("Gangnam6", 
                include_bytes!("../assets/2BGangnam.png")).unwrap());

        //7
        images.push(RetainedImage::from_image_bytes("Fresh7", 
                include_bytes!("../assets/2BFresh.png")).unwrap());

        //8
        images.push(RetainedImage::from_image_bytes("Floss8", 
                include_bytes!("../assets/2BFloss.png")).unwrap());

        Self {
            wheel_image: wheel_image,
            images: images,
            positions: Vec::new(),
            selected: 0,
        }
    }
}

impl WheelWidget {
    pub fn play_emote(&mut self) {
        eprintln!("Playing Emote: {}", self.images[self.selected as usize].debug_name());
    }

    pub fn get_selected(&mut self, ctx: &egui::Context) {
        let mut closest = 0;
        let mut least_distance = std::f32::MAX;
        for i in 0..self.positions.len() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                let distance = pos.distance(self.positions[i]);
                if distance < least_distance {
                    least_distance = distance;
                    closest = i;
                }
            }
        }

        self.selected = closest as i32;
    }

    pub fn display(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> egui::Response {
        let desired_size = ui.available_size();

        // Allocate size, and allow for clicks (apparently)
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let wheel_rect = Rect::from_center_size(rect.center(),Vec2::new(desired_size.y * 0.40 * 2.5, desired_size.y * 0.40 * 2.5));

            ui.painter()
                .image(self.wheel_image.texture_id(ctx), wheel_rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);

            let r = desired_size.y * 0.38;
            let slice = 2.0 * PI / 8.0;

            // Image texture rectangle size
            let size_x = desired_size.x * 0.035;
            let size_y = desired_size.y * 0.08;

            self.get_selected(ctx);

            for i in 0..self.images.len() {
                // Circle math
                let angle = slice * i as f32;
                let x = rect.center().x + r * angle.cos();
                let y = rect.center().y + r * angle.sin();

                let image_rect = Rect::from_min_max(pos2(x - size_x, y - size_y), pos2(x + size_x, y + size_y));

                if self.positions.len() < 8 {
                    self.positions.push(pos2(x, y));
                }

                if self.selected == i as i32 {
                    ui.painter()
                        .image(self.images[i].texture_id(ctx), image_rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::LIGHT_RED);
                }

                else {
                    ui.painter()
                        .image(self.images[i].texture_id(ctx), image_rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                }
            }

            if let Some(mouse_pos) = ctx.pointer_latest_pos() {
                let fixed_pos = (mouse_pos - rect.center()).normalized() * rect.width() * 0.02;
                ui.painter()
                    .circle(pos2(rect.center().x + fixed_pos.x, rect.center().y + fixed_pos.y), rect.width() * 0.02, Color32::GRAY, Stroke::new(1.0, Color32::BLUE));
            } else {
                ui.painter()
                    .circle(rect.center(), rect.width() * 0.02, Color32::GRAY, Stroke::new(1.0, Color32::BLUE));
            }

        };

        response
    }
}
