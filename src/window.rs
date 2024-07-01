use super::renderer::Renderer;
use super::camera::CameraMovement;


pub struct App {
    template: glutin::config::ConfigTemplateBuilder,
    display_builder: glutin_winit::DisplayBuilder,
    exit_state: Result<(), Box<dyn std::error::Error>>,
    not_current_gl_context: Option<glutin::context::NotCurrentContext>,
    renderer: Option<Renderer>,
    // NOTE: `AppState` carries the `Window`, thus it should be dropped after everything else.
    state: Option<AppState>,
    last_frame: std::time::Instant,
    initial_time: std::time::Instant,
    last_mouse: Option<winit::dpi::PhysicalPosition<f32>>,
    cursor_locked: bool,
}

struct AppState {
    gl_context: glutin::context::PossiblyCurrentContext,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    // NOTE: Window should be dropped after all resources created using its
    // raw-window-handle.
    window: winit::window::Window,
}

impl App {
    pub fn new(template: glutin::config::ConfigTemplateBuilder, display_builder: glutin_winit::DisplayBuilder) -> Self {
        Self {
            template,
            display_builder,
            exit_state: Ok(()),
            not_current_gl_context: None,
            state: None,
            renderer: None,
            last_frame: std::time::Instant::now(),
            initial_time: std::time::Instant::now(),
            last_mouse: None,
            cursor_locked: false,
        }
    }
}

pub fn gl_config_picker(
    configs: Box<dyn Iterator<Item = glutin::config::Config> + '_>,
) -> glutin::config::Config {
    use glutin::config::GlConfig;
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        use glutin::config::GlConfig;
        use glutin::context::NotCurrentGlContext;
        use glutin::display::GetGlDisplay;
        use glutin::display::GlDisplay;
        use glutin::prelude::GlSurface;
        use glutin_winit::GlWindow;
        use winit::raw_window_handle::HasWindowHandle;

        let (mut window, gl_config) = match self.display_builder.clone().build(
            event_loop,
            self.template.clone(),
            gl_config_picker,
        ) {
            Ok(ok) => ok,
            Err(e) => {
                self.exit_state = Err(e);
                event_loop.exit();
                return;
            }
        };
        println!("Picked a config with {} samples", gl_config.num_samples());

        let raw_window_handle = window
            .as_ref()
            .and_then(|window| window.window_handle().ok())
            .map(|handle| handle.as_raw());

        // XXX The display could be obtained from any object created by it, so we can
        // query it from the config.
        let gl_display = gl_config.display();

        // The context creation part.
        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                glutin::context::Version::new(4, 5),
            )))
            .build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                glutin::context::Version::new(2, 1),
            )))
            .build(raw_window_handle);

        self.not_current_gl_context.replace(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .unwrap_or_else(|_| {
                            gl_display
                                .create_context(&gl_config, &legacy_context_attributes)
                                .expect("failed to create context")
                        })
                })
        });

        #[cfg(android_platform)]
        println!("Android window available");

        let window = window.take().unwrap_or_else(|| {
            let window_attributes = winit::window::Window::default_attributes()
                .with_transparent(true)
                .with_title("Glutin triangle gradient example (press Escape to exit)");
            glutin_winit::finalize_window(event_loop, window_attributes, &gl_config).unwrap()
        });

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        // Make it current.
        let gl_context = self
            .not_current_gl_context
            .take()
            .unwrap()
            .make_current(&gl_surface)
            .unwrap();

        // The context needs to be current for the Renderer to set up shaders and
        // buffers. It also performs function loading, which needs a current context on
        // WGL.
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;
        let image_width = size.width as f32;
        self.renderer
            .get_or_insert_with(|| Renderer::new(&gl_display, aspect_ratio, image_width));

        // Try setting vsync.
        if let Err(res) = gl_surface.set_swap_interval(
            &gl_context,
            glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap()),
        ) {
            eprintln!("Error setting vsync: {res:?}");
        }


        assert!(self
            .state
            .replace(AppState {
                gl_context,
                gl_surface,
                window
            })
            .is_none());
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        use glutin::context::PossiblyCurrentGlContext;
        // This event is only raised on Android, where the backing NativeWindow for a GL
        // Surface can appear and disappear at any moment.
        println!("Android window removed");

        // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
        // the window back to the system.
        let gl_context = self.state.take().unwrap().gl_context;
        assert!(self
            .not_current_gl_context
            .replace(gl_context.make_not_current().unwrap())
            .is_none());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use glutin::prelude::GlSurface;
        use winit::{
            event::{KeyEvent, WindowEvent, ElementState, MouseButton},
            keyboard::{Key, NamedKey},
            window::CursorGrabMode,
        };

        let delta_time = self.last_frame.elapsed().as_secs_f32();


        match event {
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                // Some platforms like EGL require resizing GL surface to update the size
                // Notable platforms here are Wayland and macOS, other don't require it
                // and the function is no-op, but it's wise to resize it for portability
                // reasons.
                if let Some(AppState {
                    gl_context,
                    gl_surface,
                    window: _,
                }) = self.state.as_ref()
                {
                    gl_surface.resize(
                        gl_context,
                        std::num::NonZeroU32::new(size.width).unwrap(),
                        std::num::NonZeroU32::new(size.height).unwrap(),
                    );
                    let renderer = self.renderer.as_ref().unwrap();
                    renderer.resize(size.width as i32, size.height as i32);
                }
            }
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Character(ch),
                        ..
                    },
                ..
            } => {
                if let Some(renderer) = &mut self.renderer {
                    match ch.as_str() {
                        "w" => renderer
                            .camera
                            .process_keyboard(CameraMovement::FORWARD, delta_time),
                        "a" => renderer
                            .camera
                            .process_keyboard(CameraMovement::LEFT, delta_time),
                        "s" => renderer
                            .camera
                            .process_keyboard(CameraMovement::BACKWARD, delta_time),
                        "d" => renderer
                            .camera
                            .process_keyboard(CameraMovement::RIGHT, delta_time),
                        _ => (),
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(renderer) = &mut self.renderer {
                    if self.cursor_locked {

                        let new_position: winit::dpi::PhysicalPosition<f32> = position.cast();
                        let last_pos = self.last_mouse.unwrap_or(new_position.clone());
                        let (x_offset, y_offset) = (
                            last_pos.x - new_position.x,
                            new_position.y - last_pos.y,
                        );
                        self.last_mouse = Some(new_position);
                        renderer.camera.process_mouse_movements(x_offset, y_offset);
                    }
                }
            }
            // Tried to grab the cursor to make it not move. Can't make it work. Documetation says it returns error, but I don't see any error either. This is so confusing.
            // if let Some(app_state) = &self.state {
            //     println!("Grabbing the cursor");
            //     if let Err(err) = app_state.window.set_cursor_grab(winit::window::CursorGrabMode::Locked) {
            //         eprintln!("Error setting cursor grab: {err}");
            //     }
            // }
            WindowEvent::CursorLeft {device_id: _} => {
                self.last_mouse = None;
            }
            WindowEvent::MouseInput { state: ElementState::Pressed | ElementState::Released, button: MouseButton::Right, .. } => {
                if let Some(app_state) = &self.state {
                    let window = &app_state.window;
                    if self.cursor_locked {
                        window.set_cursor_grab(CursorGrabMode::None).unwrap();
                        window.set_cursor_visible(true);
                        self.cursor_locked = false;
                        self.last_mouse = None;
                    } else {
                        window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
                        window.set_cursor_visible(false);
                        self.cursor_locked = true;
                    }
                }
            }
            _ => (),
        }
    }
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        use glutin::prelude::GlSurface;

        if let Some(AppState {
            gl_context,
            gl_surface,
            window,
        }) = self.state.as_ref()
        {
            self.last_frame = std::time::Instant::now();
            let renderer = self.renderer.as_mut().unwrap();
            
            renderer.draw(self.initial_time.elapsed().as_secs_f32()*1000.0);
            window.request_redraw();
            
            gl_surface.swap_buffers(gl_context).unwrap();
            let delta_time = self.last_frame.elapsed().as_secs_f32()*1000.0;
            println!("{}", delta_time);
        }
    }
}
