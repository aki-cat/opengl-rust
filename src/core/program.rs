use std::ptr;

use gl::{types::GLuint, INFO_LOG_LENGTH};

use super::Shader;

/// Shader Program Object
pub struct Program {
    program: GLuint,
}

impl Program {
    pub(super) fn new() -> Self {
        let program = unsafe { gl::CreateProgram() };
        Self { program }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

impl Program {
    /// Use this shader program in the current context.
    pub fn using(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    /// Attach a shader to this program.
    pub fn attach(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.program, shader.shader);
        }
    }

    /// Link this program.
    ///
    /// It will return `Ok(())` if the program is linked successfully,
    /// otherwise it will return `Err(String)` which contains the error message.
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
    pub unsafe fn link_unchecked(&self) {
        gl::LinkProgram(self.program);
    }
}

use mats::uniform::SetUniform;
impl Program {
    pub fn set_uniform<T: SetUniform>(&self, name: &str, value: &T) -> Result<(), String> {
        let name_ptr = match std::ffi::CString::new(name) {
            Ok(c_str) => c_str,
            Err(_) => return Err(format!("Invalid uniform name '{}'", name)),
        };
        let location = unsafe { gl::GetUniformLocation(self.program, name_ptr.as_ptr()) };
        if location == -1 {
            return Err(format!("Uniform '{}' not found", name));
        }
        value.give(location);
        Ok(())
    }
}
