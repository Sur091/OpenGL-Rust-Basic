# My First GLFW Window
This project demonstrates the creation of a simple window using the GLFW library in Rust. It draws a square on the screen and responds to the escape key to close the window.

## Features
- Creates a window with a title and specified dimensions.
```rust
const TITLE: &str = "My First GLFW window";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
let mut glfw = glfw::init(fail_on_errors!()).unwrap();
let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .unwrap();
```
- Initializes OpenGL context (version 4.5 core profile).
```rust
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
```
- Loads a vertex and fragment shader from external files.
```rust
const VERT_SHADER_PATH: &str = "./src/shader/vertex_shader.glsl";

const FRAG_SHADER_PATH: &str = "./src/shader/fragment_shader.glsl";

let mut shader = Shader::new(VERT_SHADER_PATH, FRAG_SHADER_PATH);
```
- Defines a square using vertex data and indices.
```rust
const SIZE: f32 = 0.5;
const VERTICES: [f32; 12] = [
    -SIZE, -SIZE, 0.0, 
     SIZE, -SIZE, 0.0, 
     SIZE,  SIZE, 0.0,
    -SIZE,  SIZE, 0.0,
];

const INDICES: [u32; 6] = [
    0, 1, 2,
    0, 2, 3,
];

let vao = VertexArray::new();

let vbo = VertexBuffer::new(&VERTICES);

let mut layout = VertexBufferLayout::new();
layout.push_f32(3);

vao.add_buffer(&vbo, &layout);

vbo.unbind();
vao.unbind();

let ib = IndexBuffer::new(&INDICES);
ib.unbind();
```
- Renders the square using vertex and fragment shaders.
```rust
shader.bind();
vao.bind();
ib.bind();
unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::DrawElements(gl::TRIANGLES, ib.get_count(), gl::UNSIGNED_INT, 0 as *const _);
}
```
- Handles basic user input (escape key to close).
```rust
while !window.should_close() {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        glfw_handle_event(&mut window, event);
    }
    // ... (remaining code for drawing)
}

// ...
fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
```
- Optionally enables OpenGL debug messages for detailed error reporting (debug mode only).
```rust
#[cfg(debug_assertions)]
unsafe {
    gl::Enable(gl::DEBUG_OUTPUT);
    gl::DebugMessageCallback(Some(message_callback), std::ptr::null());
}

// ... 

#[allow(dead_code)]
extern "system" fn message_callback(
    source: gl::types::GLenum,
    ty: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut std::os::raw::c_void,
) {
    // ...
}
```
- Prints OpenGL and GLSL version information upon startup.

## Running the Project
- Ensure you have Rust and Cargo installed (https://www.rust-lang.org/tools/install).
- Clone or download the project repository.
- Navigate to the project directory in your terminal.
- Run cargo run to compile and execute the program.

## Dependencies
glfw: A library for creating windows, contexts, and handling input.

## Project Structure
The project uses the following modules:

index_buffer: Manages index buffer objects for efficient rendering.
shader: Loads and compiles vertex and fragment shaders.
vertex_array: Manages vertex array objects (VAOs) and vertex buffer layouts.
lib.rs: The main entry point containing the application logic.

## Contributing
Feel free to fork this repository and make your own modifications!

Note: This is a basic example, and you can extend it further by:
- Implementing additional shapes or objects.
- Adding user interaction (e.g., mouse movement).
- Incorporating more complex shaders for lighting and effects