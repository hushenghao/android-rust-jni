package com.dede.rust.jni

/**
 * Created by shhu on 2022/12/20 16:42.
 *
 * @author shhu
 * @since 2022/12/20
 */
class RustJNI {

    companion object {
        init {
            System.loadLibrary("android_rust_jni")
        }

        @JvmStatic
        external fun hello(string: String): String

        @JvmStatic
        external fun md5(string: String): String

        @JvmStatic
        external fun log(level: Int, tag: String, msg: String)
    }
}