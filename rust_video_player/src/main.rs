extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::path::Path;
use std::time::Duration;
use sdl2::rect::Rect;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let video;


    //Creating tha Window
    let window = video_subsystem.window("Video player", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create Render-Canvas
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();


    fn load_video<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        file_path: &str,
    ) -> Texture<'a> {
        use std::path::Path;

        // Lade das Video als Surface
        let surface = match Surface::load_bmp(Path::new(file_path)) {
            Ok(s) => s,
            Err(e) => panic!("Fehler beim Laden des Videos: {}", e),
        };

        // Konvertiere die Surface in eine Textur
        match texture_creator.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(e) => panic!("Fehler beim Erstellen der Textur: {}", e),
        }
    }

    let mut texture = load_video(&texture_creator, "/users/teric.marko/Desktop/");



    'run: loop {

        // Eventbehandlung
        'running: for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Clear Canvas
        canvas.clear();

        // Rendern der Video-Textur
        canvas.copy(&texture, None, Some(Rect::new(0, 0, 800, 600))).unwrap();

        // Aktualisieren der Anzeige
        canvas.present();

        // Warten für die nächste Bildwiederholung
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }



    // Main loop
    // 'mainloop: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'mainloop,
    //             _ => {}
    //         }
    //     }

        // Clear canvas
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Render canvas
        canvas.present();
    }

