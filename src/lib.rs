use android_log_sys::{
    LogPriority::{FATAL, INFO, VERBOSE},
    __android_log_write,
};
use jni::{
    objects::{JClass, JString},
    sys::{jint, jstring},
    JNIEnv,
};
use std::cmp::{max, min};
use std::ffi::CString;

const TAG: &str = "RustJNI";

/// JNI.
#[no_mangle]
pub extern "C" fn Java_com_dede_rust_jni_RustJNI_hello(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input = JNIConverter::get_string(env, input);
    android_log!(INFO, TAG, format!("Hello {}!", input));
    let format = format!("Hello {}!", input);
    let output = JNIConverter::to_jstring(env, format);
    return output;
}

/// JNI.
#[no_mangle]
pub extern "C" fn Java_com_dede_rust_jni_RustJNI_md5(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input = JNIConverter::get_string(env, input);
    let md5 = Utils::md5(input);
    let output = JNIConverter::to_jstring(env, md5);
    return output;
}

/// JNI.
#[no_mangle]
pub extern "C" fn Java_com_dede_rust_jni_RustJNI_log(
    env: JNIEnv,
    _class: JClass,
    level: jint,
    tag: JString,
    msg: JString,
) {
    let tag = JNIConverter::get_string(env, tag);
    let msg = JNIConverter::get_string(env, msg);
    let level: i32 = max(VERBOSE as _, min(level as _, FATAL as _));
    android_log!(level, tag.to_string(), msg.to_string());
}

#[macro_export]
macro_rules! android_log {
    ($level:expr, $tag:expr, $msg:expr) => {
        unsafe {
            let tag = CString::new($tag).unwrap();
            let msg = CString::new($msg).unwrap();
            __android_log_write($level as i32, tag.as_ptr(), msg.as_ptr());
        }
    };
}

pub struct Utils;

impl Utils {
    pub fn md5_str(s: &str) -> String {
        return Utils::md5(s.into());
    }

    pub fn md5(s: String) -> String {
        let digest = md5::compute(s);
        return format!("{:02X}", digest);
    }
}

struct JNIConverter;

impl JNIConverter {
    fn get_string(env: JNIEnv, input: JString) -> String {
        if input.is_null() {
            // error!("jstring null ptr");
            return "".into();
        }
        return env
            .get_string(input)
            .expect("Couldn't get java string!")
            .into();
    }

    fn to_jstring(env: JNIEnv, input: String) -> jstring {
        return env
            .new_string(input)
            .expect("Couldn't create java string!")
            .into_raw();
    }
}
