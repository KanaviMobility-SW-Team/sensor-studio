package com.example.sensorstudiocore

import android.app.Activity
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.Gravity
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import android.widget.Toast

class MainActivity : Activity() {

    private lateinit var statusText: TextView
    private lateinit var startButton: Button
    private lateinit var stopButton: Button

    private var transitionInProgress = false

    private val handler = Handler(Looper.getMainLooper())

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        statusText = TextView(this).apply {
            textSize = 18f
            gravity = Gravity.CENTER
        }

        startButton = Button(this).apply {
            text = "서비스 시작"

            setOnClickListener {
                startCoreService()
            }
        }

        stopButton = Button(this).apply {
            text = "서비스 종료"

            setOnClickListener {
                stopCoreService()
            }
        }

        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            gravity = Gravity.CENTER
            setPadding(48, 48, 48, 48)

            addView(
                statusText,
                LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.MATCH_PARENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                )
            )

            addView(
                startButton,
                LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.MATCH_PARENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                )
            )

            addView(
                stopButton,
                LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.MATCH_PARENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                )
            )
        }

        setContentView(layout)
        updateUi()
    }

    override fun onResume() {
        super.onResume()
        updateUi()
    }

    private fun startCoreService() {
        if (transitionInProgress) {
            return
        }

        if (RustForegroundService.isRunning) {
            Toast.makeText(
                this,
                "서비스가 이미 실행 중입니다",
                Toast.LENGTH_SHORT
            ).show()

            updateUi()
            return
        }

        transitionInProgress = true
        updateUi()

        val intent = Intent(
            this,
            RustForegroundService::class.java
        )

        try {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                startForegroundService(intent)
            } else {
                startService(intent)
            }

            Toast.makeText(
                this,
                "서비스 시작 요청",
                Toast.LENGTH_SHORT
            ).show()
        } catch (error: Exception) {
            transitionInProgress = false

            Toast.makeText(
                this,
                "서비스 시작 실패: ${error.message}",
                Toast.LENGTH_LONG
            ).show()
        }

        handler.postDelayed(
            {
                transitionInProgress = false
                updateUi()
            },
            1000L
        )
    }

    private fun stopCoreService() {
        if (transitionInProgress) {
            return
        }

        if (!RustForegroundService.isRunning) {
            Toast.makeText(
                this,
                "실행 중인 서비스가 없습니다",
                Toast.LENGTH_SHORT
            ).show()

            updateUi()
            return
        }

        transitionInProgress = true
        updateUi()

        val intent = Intent(
            this,
            RustForegroundService::class.java
        )

        val stopped = stopService(intent)

        Toast.makeText(
            this,
            if (stopped) {
                "서비스 종료 요청"
            } else {
                "서비스를 종료하지 못했습니다"
            },
            Toast.LENGTH_SHORT
        ).show()

        handler.postDelayed(
            {
                transitionInProgress = false
                updateUi()
            },
            1000L
        )
    }

    private fun updateUi() {
        val running = RustForegroundService.isRunning

        statusText.text = when {
            transitionInProgress -> "상태 변경 중..."
            running -> "서비스 실행 중"
            else -> "서비스 정지"
        }

        startButton.isEnabled =
            !transitionInProgress && !running

        stopButton.isEnabled =
            !transitionInProgress && running
    }
}