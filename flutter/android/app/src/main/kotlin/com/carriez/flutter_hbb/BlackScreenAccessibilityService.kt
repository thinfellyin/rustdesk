package com.carriez.flutter_hbb

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent

class BlackScreenAccessibilityService : AccessibilityService() {
    override fun onAccessibilityEvent(event: AccessibilityEvent?) {}
    override fun onInterrupt() {}
} 