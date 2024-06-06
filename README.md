### My First GLFW Window
This project demonstrates the creation of a simple window using the GLFW library in Rust. It draws a square on the screen and responds to the escape key to close the window.

## Features
- Creates a window with a title and specified dimensions.
``rust`let mut glfw = glfw::init(fail_on_errors!()).unwrap();
let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .unwrap();

- Initializes OpenGL context (version 4.5 core profile).
- Loads a vertex and fragment shader from external files.
- Defines a square using vertex data and indices.
- Renders the square using vertex and fragment shaders.
- Handles basic user input (escape key to close).
- Optionally enables OpenGL debug messages for detailed error reporting (debug mode only).
- Prints OpenGL and GLSL version information upon startup.

##Running the Project
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