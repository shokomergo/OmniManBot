use eframe::{egui, App};
use egui::{Color32, RichText, ScrollArea, Vec2};
use image::ImageReader;
use image::imageops::FilterType;

mod quotes;
use quotes::OMNI_MAN_QUOTES;

use quotes::omni_man_reply;

#[derive(Clone)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Clone)]
enum Role {
    User,
    Bot,
}

struct OmniManBot {
    input_value: String,
    messages: Vec<Message>,
    bot_avatar: Option<egui::TextureHandle>,
}

impl Default for OmniManBot {
    fn default() -> Self {
        Self {
            input_value: String::new(),
            messages: vec![],
            bot_avatar: None,
        }
    }
}

impl App for OmniManBot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.bot_avatar.is_none() {
            let img = match ImageReader::open("assets/Are_You_Sure.png") {
                Ok(reader) => match reader.decode() {
                    Ok(img) => img,
                    Err(e) => return,
                },
                Err(_) => return,
            };

            let img_small = img.resize_exact(32, 32, FilterType::Triangle);

            let size = [img_small.width() as usize, img_small.height() as usize];
            let rgba = img_small.to_rgba8().into_vec();

            self.bot_avatar = Some(ctx.load_texture(
                "bot_avatar",
                egui::ColorImage::from_rgba_unmultiplied(size, &rgba),
                egui::TextureOptions::LINEAR,
            ));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui|  {
                for msg in &self.messages {
                    match msg.role {
                        Role::User => {
                            egui::Frame::none()
                                .fill(Color32::from_rgb(0, 120, 255))
                                .rounding(5.0)
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(&msg.content).color(Color32::WHITE));
                                });
                        }
                        Role::Bot => {
                            ui.horizontal(|ui| {
                                if let Some(tex) = &self.bot_avatar {
                                    ui.add_sized([32.0, 32.0], egui::Image::new(tex));
                                }
                                
                                egui::Frame::none()
                                    .fill(Color32::from_rgb(200, 200, 200))
                                    .rounding(5.0)
                                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                    .show(ui, |ui| {
                                        ui.label(RichText::new(&msg.content).color(Color32::BLACK));
                                    });
                            });

                        }
                    }

                    ui.add_space(5.0);
                }

            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let input = ui.text_edit_singleline(&mut self.input_value);
                
                if ui.button("Send").clicked()
                    || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    if !self.input_value.trim().is_empty() {
                        self.messages.push(Message {
                            role: Role::User,
                            content: self.input_value.clone(),
                        });
                    
                    let bot_reply = omni_man_reply(&self.input_value);
                    self.messages.push(Message {
                        role: Role::Bot,
                        content: bot_reply,
                    });

                    self.input_value.clear();
                    input.request_focus();
                    }
                }
            })
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Omni-Man Bot",
        options,
        Box::new(|_cc| Box::new(OmniManBot::default())),
    )
}