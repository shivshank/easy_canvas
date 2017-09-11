use shape::DrawCmd;
use color::Rgba;

use cgmath::Matrix4;

use gl;
use gl::types::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::os::raw::c_void;
use std::mem::size_of_val;
use std::i16;

/// Default Vertex Shader used for rendering objects to a canvas layer
static VERTEX_SHADER: &'static str = r#"
    #version 330 core

    // uniform mat4 proj_matrix;
    // uniform mat4 cam_matrix;

    layout (location = 0) in vec3 pos;
    layout (location = 1) in vec3 color;
    layout (location = 2) in vec2 uv;

    out vec3 pass_color;
    out vec2 pass_uv;

    void main(void) {
        gl_Position = vec4(pos, 1.0);

        pass_color = color;
        pass_uv = uv;
    }
"#;

/// Default Fragment Shader used for rendering objects to a canvas layer
static FRAGMENT_SHADER: &'static str = r#"
    #version 330 core

    in vec3 pass_color;
    in vec2 pass_uv;

    out vec4 FragColor;

    uniform sampler2D diffuse;

    void main(void) {
        FragColor = texture(diffuse, pass_uv) * vec4(pass_color, 1.0);
    }
"#;

/// Framebuffer object
pub type Fbo = GLuint;
pub type Tex = GLuint;
/// Renderbuffer object
pub type Rbo = GLuint;
/// Vertex array object
pub type Vao = GLuint;
pub type BuffObj = GLuint;
pub type GlProgram = GLuint;
pub type Shader = GLuint;

pub struct GlRenderTarget {
    width: i32,
    height: i32,
    ms_fbo: Fbo,
    ms_tex: Tex,
    ms_rbo: Rbo,
    fbo: Fbo,
    tex: Tex,
    diffuse_loc: GLint,
    screen_program: GlProgram,
    screen_quad_vao: Vao,
    screen_quad_vbo: BuffObj,
}

#[repr(C)]
pub struct Vertex([f32; 3], [u8; 4], [i16; 2]);

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum GlError {
    /// No error has been recorded. The value of this symbolic constant is guaranteed to be 0.
    NO_ERROR = gl::NO_ERROR,
    /// An unacceptable value is specified for an enumerated argument. The offending command is ignored and has no other side effect than to set the error flag.
    INVALID_ENUM = gl::INVALID_ENUM,
    /// A numeric argument is out of range. The offending command is ignored and has no other side effect than to set the error flag.
    INVALID_VALUE = gl::INVALID_VALUE,
    /// The specified operation is not allowed in the current state. The offending command is ignored and has no other side effect than to set the error flag.
    INVALID_OPERATION = gl::INVALID_OPERATION,
    /// The command is trying to render to or read from the framebuffer while the currently bound framebuffer is not framebuffer complete (i.e. the return value from glCheckFramebufferStatus is not FRAMEBUFFER_COMPLETE = gl::FRAMEBUFFER_COMPLETE). The offending command is ignored and has no other side effect than to set the error flag.
    INVALID_FRAMEBUFFER_OPERATION = gl::INVALID_FRAMEBUFFER_OPERATION,
    OUT_OF_MEMORY = gl::OUT_OF_MEMORY,
}

macro_rules! get_info_log {
    ($get_attr:ident, $get_log:ident, $gl_id:expr) => {{
        let mut log_length_glint: GLint = 0;
        $get_attr($gl_id, gl::INFO_LOG_LENGTH, &mut log_length_glint);
        let log_length = log_length_glint as usize;
        if log_length == 0 {
            None
        } else {
            let mut raw_log = Vec::<u8>::with_capacity(log_length);
            $get_log($gl_id, log_length as GLsizei,
                0 as *mut GLsizei, raw_log.as_mut_ptr() as *mut GLchar);
            raw_log.set_len(log_length);
            let log = String::from_utf8(raw_log)
                .expect("OpenGL returned invalid utf8 in a program info log");
            Some(log)
        }
    }}
}

/// Creates a program.
fn create_program(vert_shader: Shader, frag_shader: Shader) -> GlProgram {
    unsafe {
        let gl_id = gl::CreateProgram();
        if gl_id == 0 {
            panic!("Failed to create program");
        }
        gl::AttachShader(gl_id, vert_shader);
        gl::AttachShader(gl_id, frag_shader);
        gl::LinkProgram(gl_id);
        // if shaders are left attached they will never be deleted until the program is deleted
        gl::DetachShader(gl_id, vert_shader);
        gl::DetachShader(gl_id, frag_shader);

        let mut link_status = 0;
        gl::GetProgramiv(gl_id, gl::LINK_STATUS, &mut link_status);
        if link_status != 1 {
            use gl::{GetProgramiv, GetProgramInfoLog};
            match get_info_log!(GetProgramiv, GetProgramInfoLog, gl_id) {
                Some(log) => {
                    panic!("Program creation failed with the following log:\n\n{}\n", log);
                }
                None => {
                    panic!("Program creation failed with no log provided.");
                }
            }
        }

        let mut status = -1;
        gl::ValidateProgram(gl_id);
        gl::GetProgramiv(gl_id, gl::VALIDATE_STATUS, &mut status);
        if status != 1 {
            use gl::{GetProgramiv, GetProgramInfoLog};
            match get_info_log!(GetProgramiv, GetProgramInfoLog, gl_id) {
                Some(log) => {
                    panic!("Program creation failed with the following log:\n\n{}\n", log);
                }
                None => {
                    panic!("Program creation failed with no log provided.");
                }
            }
        }

        gl_id
    }
}

fn create_shader(kind: GLenum, source: &str) -> Shader {
    unsafe {
        let gl_id = gl::CreateShader(kind as u32);
        if gl_id == 0 {
            panic!("Failed to create new shader!")
        }
        gl::ShaderSource(gl_id, 1, &(source.as_ptr() as *const i8), &(source.len() as i32));
        gl::CompileShader(gl_id);

        let mut status = 0;
        gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &mut status);
        if status != 1 {
            use gl::{GetShaderiv, GetShaderInfoLog};
            match get_info_log!(GetShaderiv, GetShaderInfoLog, gl_id) {
                Some(log) => {
                    panic!("Shader creation failed with the following log:\n{}", log);
                }
                None => {
                    panic!("Shader creation failed with no log provided.");
                }
            }
        }
        gl_id
    }
}

fn delete_shader(shader: Shader) {
    unsafe {
        gl::DeleteShader(shader);
    }
}

/// Create a vao along with a corresponding vertex buffer object.
fn create_vao() -> (Vao, BuffObj) {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        declare_format();

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        (vao, vbo)
    }
}

fn declare_format() {
    unsafe {
        /*
        layout {
            position (3 floats, 0 offset)
            color (4 normalized bytes, 12 offset)
            uvs (2 normalized shorts, 16 offset)
            (stride = 20)
        }
        */
        let stride = 20;
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        gl::EnableVertexAttribArray(2);
        // position
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as * const _);
        // color
        gl::VertexAttribPointer(1, 4, gl::UNSIGNED_BYTE, gl::TRUE, stride, 12 as * const _);
        // uv
        gl::VertexAttribPointer(2, 2, gl::UNSIGNED_SHORT, gl::TRUE, stride, 16 as * const _);
    }
}

fn make_projection_matrix(width: u32, height: u32, _layers: u32) -> Matrix4<f32> {
    // TODO: Add support for rendering at discrete depths
    assert_ne!(_layers, 0, "Cannot create a canvas with 0 layers");
    let (width, height) = (width as f32, height as f32);
    Matrix4::new(
        2.0 / width, 0.0, 0.0, 0.0,
        0.0, 2.0 / height, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Create a multi-sampled, FBO with depth, stencil, and RGBA color attachments
fn create_ms_cds_render_target(width: i32, height: i32) -> (Fbo, Tex, Rbo) {
    let samples = 4;
    unsafe {
        let mut ms_fbo = 0;
        gl::GenFramebuffers(1, &mut ms_fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, ms_fbo);

        // create a color attachment texture
        let mut color_attach = 0;
        gl::GenTextures(1, &mut color_attach);
        gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, color_attach);
        gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, samples, gl::RGBA, width,
            height, gl::TRUE);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D_MULTISAMPLE, color_attach, 0);

        // create a renderbuffer to be used as depth and stencil
        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, samples, gl::DEPTH24_STENCIL8,
            width, height);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER, rbo);

        match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
            gl::FRAMEBUFFER_COMPLETE => {},
            err => panic!("framebuffer is incomplete, status: {}", err)
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        (ms_fbo, color_attach, rbo)
    }
}

fn create_color_render_target(width: i32, height: i32) -> (Fbo, Tex) {
    unsafe {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        // create a color attachment texture
        let mut color_attach = 0;
        gl::GenTextures(1, &mut color_attach);
        gl::BindTexture(gl::TEXTURE_2D, color_attach);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA,
        gl::UNSIGNED_BYTE, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D,
            color_attach, 0);

        match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
            gl::FRAMEBUFFER_COMPLETE => {},
            err => panic!("framebuffer is incomplete, status: {}", err)
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        (fbo, color_attach)
    }
}

/// Creates a Framebuffer with a 2d texture and depth/stencil renderbuffer attachments
///
/// Note that the texture is unitialized so the result is undefined if you use the texture before
/// rendering to it.
pub fn create_render_target(width: i32, height: i32) -> GlRenderTarget {
    let (ms_fbo, ms_tex, ms_rbo) = create_ms_cds_render_target(width, height);
    let (fbo, tex) = create_color_render_target(width, height);
    let (screen_quad_vao, screen_quad_vbo) = create_vao();
    let screen_program = {
        let vert = create_shader(gl::VERTEX_SHADER, VERTEX_SHADER);
        let frag = create_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER);
        let program = create_program(vert, frag);
        delete_shader(vert);
        delete_shader(frag);
        program
    };
    let diffuse_loc;
    unsafe {
        diffuse_loc = gl::GetUniformLocation(screen_program,
            b"diffuse\0".as_ptr() as *const _);
        gl::UseProgram(screen_program);
        gl::Uniform1i(diffuse_loc, 0);
        gl::UseProgram(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, screen_quad_vbo);
        // counter clockwise
        let data = [
            Vertex ([-1.0, -1.0, 0.0f32], [255, 255, 255, 255u8], [0, i16::MAX]),
            Vertex ([1.0, -1.0, 0.0f32], [255, 255, 255, 255u8], [i16::MAX, i16::MAX]),
            Vertex ([1.0, 1.0, 0.0f32], [255, 255, 255, 255u8], [i16::MAX, 0]),

            Vertex ([1.0, 1.0, 0.0f32], [255, 255, 255, 255u8], [i16::MAX, 0]),
            Vertex ([-0.5, 1.0, 0.0f32], [255, 255, 255, 255u8], [0, 0]),
            Vertex ([-1.0, -1.0, 0.0f32], [255, 255, 255, 255u8], [0, i16::MAX]),
        ];
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&data) as isize,
            &data[0] as *const _ as *const c_void, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    GlRenderTarget {
        width,
        height,
        ms_fbo,
        ms_tex,
        ms_rbo,
        fbo,
        tex,
        diffuse_loc,
        screen_program,
        screen_quad_vao,
        screen_quad_vbo,
    }
}

pub fn delete_render_target(target: GlRenderTarget) {
    unsafe {
        let GlRenderTarget {
            width: _,
            height: _,
            ms_fbo,
            ms_tex,
            ms_rbo,
            fbo,
            tex,
            diffuse_loc,
            screen_program,
            screen_quad_vao,
            screen_quad_vbo,
        } = target;

        gl::UseProgram(0);
        gl::DeleteProgram(screen_program);

        gl::BindVertexArray(0);
        gl::DeleteVertexArrays(1, &screen_quad_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::DeleteBuffers(1, &screen_quad_vbo);

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::DeleteFramebuffers(2, &[ms_fbo, fbo] as *const _);

        gl::DeleteRenderbuffers(1, &ms_rbo);

        gl::DeleteTextures(2, &[ms_tex, tex] as *const _);
    }
}

pub fn use_ms_render_target(target: &GlRenderTarget) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, target.ms_fbo);
    }
}

pub fn update_flat_target(target: &GlRenderTarget) {
    unsafe {
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, target.ms_fbo);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, target.fbo);
        gl::BlitFramebuffer(0, 0, target.width, target.height, 0, 0, target.width, target.height,
            gl::COLOR_BUFFER_BIT, gl::NEAREST);
    }
}

pub fn use_flat_render_target(target: &GlRenderTarget) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, target.fbo);
    }
}

pub fn use_default_target() {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}

pub fn clear(color: Rgba) {
    unsafe {
        let (r, g, b, a) = color;
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
    }
}
pub fn enable_blending() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::BlendEquation(gl::FUNC_ADD);
    }
}

/// Parse DrawCmds into OpenGL commands.
///
/// Updates the multi-sampled FBO and copies the result to the "flat" FBO.
///
/// Enables blending and Depth test.
pub fn parse_commands(target: &GlRenderTarget, rx: &Receiver<DrawCmd>) -> bool {
    enable_blending();
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    use_ms_render_target(target);
    let mut stale = false;
    while let Ok(cmd) = rx.try_recv() {
        stale = true;
        match cmd {
            DrawCmd::Clear(c) => {
                clear(c);
            },
            _ => {}
        }
    }
    if stale {
        update_flat_target(target);
        use_default_target();
    }
    stale
}

/// Render the flat color texture to whatever framebuffer is currently bound.
///
/// Clears the current framebuffer to black.
///
/// Enables blending and disables the depth test.
pub fn draw_flat_target(target: &GlRenderTarget) {
    unsafe {
        enable_blending();
        gl::Disable(gl::DEPTH_TEST);
        clear((0.0, 0.0, 0.0, 1.0));
        gl::UseProgram(target.screen_program);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, target.tex);

        gl::BindVertexArray(target.screen_quad_vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
    }
}

pub fn print_gl_error(at: &'static str) {
    unsafe {
        let error = gl::GetError();
        let error: GlError = ::std::mem::transmute(error);
        if error != GlError::NO_ERROR {
            println!("Got GlError at {}: {:?}", at, error);
        }
    }
}
