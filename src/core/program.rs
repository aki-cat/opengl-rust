use std::ptr;

use gl::{types::GLuint, INFO_LOG_LENGTH};

use super::Shader;

/// Shader Program Object
pub struct Program {
    program: GLuint,
}

impl Program {
    #[inline]
    pub(super) fn new() -> Self {
        let program = unsafe { gl::CreateProgram() };
        Self { program }
    }
}

impl Drop for Program {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

impl Program {
    /// Use this shader program in the current context.
    #[inline]
    pub fn using(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    /// Attach a shader to this program.
    #[inline]
    pub fn attach(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.program, shader.shader);
        }
    }

    /// Link this program.
    ///
    /// It will return `Ok(())` if the program is linked successfully,
    /// otherwise it will return `Err(String)` which contains the error message.
    #[inline]
    pub fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.program);
        }
        let mut sign = 0;
        unsafe {
            gl::GetProgramiv(self.program, gl::LINK_STATUS, &mut sign);
        }
        if sign == 0 {
            let mut message_len = 0;
            unsafe {
                gl::GetProgramiv(self.program, INFO_LOG_LENGTH, &mut message_len);
            }
            let mut msg = vec![0u8; message_len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    self.program,
                    message_len,
                    ptr::null_mut(),
                    msg.as_mut_ptr() as _,
                );
            }
            let err = unsafe { String::from_utf8_unchecked(msg) };
            return Err(err);
        }
        Ok(())
    }

    /// Link this program without checking the result.
    ///
    /// If `self.link().unwarp()` is never panic,
    /// you can use `unsafe { self.link_unchecked() }` to improve performance.
    #[inline]
    pub unsafe fn link_unchecked(&self) {
        gl::LinkProgram(self.program);
    }
}

use mats::uniform::SetUniform;
impl Program {
    #[inline]
    pub fn set_uniform<T: SetUniform>(&self, name: &str, value: &T) -> Result<(), String> {
        let location = self.location(name)?;
        value.give(location);
        check_uniform_err()
    }

    pub fn set_uniform_matrix(&self, name: &str, value: &[f32; 16]) -> Result<(), String> {
        let location = self.location(name)?;
        unsafe {
            gl::ProgramUniformMatrix4fv(self.program, location, 1, gl::FALSE, value.as_ptr());
            check_uniform_err()
        }
    }

    fn location(&self, name: &str) -> Result<i32, String> {
        let name_ptr = match std::ffi::CString::new(name) {
            Ok(c_str) => c_str,
            Err(_) => return Err(format!("Invalid uniform name '{}'", name)),
        };
        let location = unsafe { gl::GetUniformLocation(self.program, name_ptr.as_ptr()) };
        if location == -1 {
            return Err(format!("Uniform '{}' not found", name));
        }
        Ok(location)
    }
}

fn check_uniform_err() -> Result<(), String> {
    let (err_enum, err) = unsafe {
        let err_enum = gl::GetError();
        match err_enum {
            gl::NO_ERROR => return Ok(()),
            gl::INVALID_ENUM => (err_enum, "GL_INVALID_ENUM"),
            gl::INVALID_VALUE => (err_enum, "GL_INVALID_VALUE"),
            gl::INVALID_OPERATION => (err_enum, "GL_INVALID_OPERATION"),
            _ => (err_enum, "Unknown Error"),
        }
    };
    Err(format!("Set Uniform Error: {}({})", err, err_enum))
}
