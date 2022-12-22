package com.dede.rust.jni

import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.dede.rust.jni.RustJNI

class MainActivity : AppCompatActivity() {

    companion object {
        private const val TAG = "MainActivity"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        RustJNI.log(1, TAG, "Hello World!")
        RustJNI.log(Log.VERBOSE, TAG, "Hello World!")
        RustJNI.log(Log.INFO, TAG, "Hello World!")
        RustJNI.log(Log.DEBUG, TAG, "Hello World!")
        RustJNI.log(Log.WARN, TAG, "Hello World!")
        RustJNI.log(Log.ERROR, TAG, "Hello World!")
        RustJNI.log(Log.ASSERT, TAG, "Hello World!")
        RustJNI.log(8, TAG, "Hello World!")
    }

    fun rustJNI(view: View) {
        Toast.makeText(this, RustJNI.hello("World"), Toast.LENGTH_SHORT).show()
        assert(RustJNI.md5("World").equals("f5a7924e621e84c9280a9a27e1bcb7f6", true))
    }
}