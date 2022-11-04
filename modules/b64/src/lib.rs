extern crate base64;
extern crate boson;

use base64::{decode, encode};

use boson::types::{
    buffer::Buffer, dyn_module::DynamicModuleInternalError, dyn_module::DynamicModuleResult,
    object::Object,
};

use std::cell::RefCell;
use std::rc::Rc;

#[no_mangle]
pub fn open(_init_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn close(_close_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn write(_write_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn read(_read_params: Rc<Object>) -> DynamicModuleResult {
    Ok(Rc::new(Object::Noval))
}

#[no_mangle]
pub fn exec(method: String, params: &Vec<Rc<Object>>) -> DynamicModuleResult {
    if params.len() != 1 {
        return Err(DynamicModuleInternalError {
            cause: format!("no_param"),
            message: format!(
                "b64 encode() or decode() expects 1 parameter, but got {}",
                params.len()
            ),
        });
    }

    let param_ref = params[0].as_ref();

    match method.as_ref() {
        "encode" => match param_ref {
            Object::ByteBuffer(buffer) => {
                let encoded = encode(&buffer.borrow().data);
                return Ok(Rc::new(Object::Str(encoded)));
            }
            _ => {
                return Err(DynamicModuleInternalError {
                    cause: format!("invalid_type"),
                    message: format!(
                        "encode() requires parameter of type 'bytes' but got {}",
                        param_ref.get_type()
                    ),
                })
            }
        },
        "decode" => match param_ref {
            Object::Str(string) => {
                let result = decode(&string);
                if result.is_err() {
                    return Err(DynamicModuleInternalError {
                        cause: format!("internal_error"),
                        message: format!("error={:?}", result.unwrap_err()),
                    });
                }

                return Ok(Rc::new(Object::ByteBuffer(RefCell::new(Buffer::from_u8(
                    result.unwrap(),
                    format!("none"),
                    true,
                )))));
            }
            _ => {
                return Err(DynamicModuleInternalError {
                    cause: format!("invalid_type"),
                    message: format!(
                        "decode() requires parameter of type 'string' but got {}",
                        param_ref.get_type()
                    ),
                })
            }
        },
        _ => {
            return Err(DynamicModuleInternalError {
                cause: format!("no_method"),
                message: format!("method {} not found", method),
            })
        }
    }
}
