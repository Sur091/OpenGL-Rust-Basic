
// use cgf_aliases::cgf_aliases;

mod camera;
mod index_buffer;
mod renderer;
mod shader;
mod texture;
mod vertex_array;
mod vertex_buffer;
mod window;
mod light;

use window::App;


const TITLE: &str = "My First GLFW window";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window_attributes = winit::window::Window::default_attributes()
        .with_transparent(true)
        .with_title(TITLE)
        .with_inner_size(size);
    let template = glutin::config::ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));
    let display_builder =
        glutin_winit::DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    let mut app = App::new(template, display_builder);
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}