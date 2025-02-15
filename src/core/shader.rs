use std::ptr;

use gl::types::GLuint;

use super::enums::ShaderType as Type;

/// Shader Object
pub struct Shader {
    pub(super) shader: GLuint,
}

impl Shader {
    /// Create a new shader which type of `ty`.
    #[inline]
    pub(super) fn new(ty: Type) -> Self {
        let shader = unsafe { gl::CreateShader(ty.to_gl_type()) };
        Self { shader }
    }
}

impl Drop for Shader {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}

impl Shader {
    /// Load source code to this shader.
    #[inline]
    pub fn source(&self, code: &str) {
        let len = code.len() as i32;
        let code = code.as_ptr() as _;
        unsafe {
            gl::ShaderSource(self.shader, 1, &code, &len);
        }
    }

    /// Compile this shader.
    ///
    /// It will return `Ok(())` if the shader is compiled successfully,
    /// otherwise it will return `Err(String)` which contains the error message.
    #[inline]
    pub fn compile(&self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.shader);
        }
        let mut sign = 0;
        unsafe {
            gl::GetShaderiv(self.shader, gl::COMPILE_STATUS, &mut sign);
        }
        if sign == 0 {
            let mut message_len = 0;
            unsafe {
                gl::GetShaderiv(self.shader, gl::INFO_LOG_LENGTH, &mut message_len);
            }
            let mut msg = vec![0u8; message_len as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    self.shader,
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

    /// Compile this shader without checking the result.
    ///
    /// If `self.compile().unwarp()` is never panic,
    /// you can use `unsafe { self.compile_unchecked() }` to improve performance.
    #[inline]
    pub unsafe fn compile_unchecked(&self) {
        gl::CompileShader(self.shader);
    }
}
