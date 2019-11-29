use sdl2::video::GLProfile;
use std::ffi::{CStr, CString};
use std::fs;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// const vertices: [f32; 32] = [
//     0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.6, 0.4, // top right
//     0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.6, 0.6, // bottom right
//     -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.4, 0.6, // bottom left
//     -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.4, 0.4, // top left
// ];

// Triangle
const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0, // left
    0.5, -0.5, 0.0, // right
    0.0, 0.5, 0.0, // top
];

pub fn start_opengl_test() {
    let vertex_source =
        fs::read_to_string("src/opengl_test/vertex.glsl").expect("Error reading file vertex.glsl");
    let fragment_source = fs::read_to_string("src/opengl_test/fragment.glsl")
        .expect("Error reading file fragment.glsl");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("opengl_test", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // Shader creation
    // vertex shader
    let vertex_shader_source: CString =
        CString::new(vertex_source.to_string()).expect("CString::new failed");

    let fragment_shader_source: CString =
        CString::new(fragment_source.to_string()).expect("CString::new failed");

    let mut vertex_shader: u32 = 0;
    let mut fragment_shader: u32 = 0;
    let mut shader_program: u32 = 0;
    unsafe {
        vertex_shader = shader_from_source(&vertex_shader_source, gl::VERTEX_SHADER).unwrap();
        fragment_shader = shader_from_source(&fragment_shader_source, gl::FRAGMENT_SHADER).unwrap();
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors LATER
        // gl::GetProgramiv(shader_program, gl::LINK_STATUS, &success);
        // if (!success) {
        //     glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
        //     std::cout << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::endl;
        // }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    // ===============

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTICES.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();

    'game: loop {
        unsafe {
            gl::ClearColor(1.0, 0.0, 0.21, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'game,
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(20));
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
        [b' '].iter().cycle().take(len as usize);
    }

    Ok(id)
}
