package com.example.sensorstudiocore

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.os.Build
import android.os.IBinder
import android.util.Log
import java.io.File

class RustForegroundService : Service() {

    companion object {
        private const val TAG = "SensorStudioCore"
        private const val CHANNEL_ID = "sensor_studio_core"
        private const val CHANNEL_NAME = "Sensor Studio Core"
        private const val NOTIFICATION_ID = 1001

        @Volatile
        var isRunning: Boolean = false
            private set

        init {
            System.loadLibrary("sensor_studio_core")
        }
    }

    private external fun nativeStart(filesDir: String)

    private external fun nativeStop()

    override fun onCreate() {
        super.onCreate()

        Log.i(TAG, "Service onCreate()")

        createNotificationChannel()

        startForeground(
            NOTIFICATION_ID,
            createNotification()
        )

        try {
            prepareRuntimeConfig()

            Log.i(TAG, "filesDir=${filesDir.absolutePath}")
            Log.i(
                TAG,
                "nativeLibraryDir=${applicationInfo.nativeLibraryDir}"
            )

            Log.i(TAG, "Calling nativeStart()")

            nativeStart(filesDir.absolutePath)

            isRunning = true

            Log.i(TAG, "nativeStart() returned")
        } catch (error: Exception) {
            isRunning = false
            Log.e(TAG, "Failed to start Sensor Studio Core", error)
            stopSelf()
        }
    }

    override fun onStartCommand(
        intent: Intent?,
        flags: Int,
        startId: Int
    ): Int {
        Log.i(TAG, "Service onStartCommand()")

        return START_STICKY
    }

    override fun onDestroy() {
        Log.i(TAG, "Service onDestroy()")

        try {
            // nativeStop()
        } catch (error: Exception) {
            Log.e(TAG, "Failed to stop Rust core", error)
        } finally {
            isRunning = false
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            stopForeground(STOP_FOREGROUND_REMOVE)
        } else {
            @Suppress("DEPRECATION")
            stopForeground(true)
        }

        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? {
        return null
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return
        }

        val channel = NotificationChannel(
            CHANNEL_ID,
            CHANNEL_NAME,
            NotificationManager.IMPORTANCE_LOW
        ).apply {
            description = "Sensor Studio Core foreground service"
            setShowBadge(false)
        }

        val notificationManager =
            getSystemService(NotificationManager::class.java)

        notificationManager.createNotificationChannel(channel)
    }

    private fun createNotification(): Notification {
        val builder =
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                Notification.Builder(this, CHANNEL_ID)
            } else {
                @Suppress("DEPRECATION")
                Notification.Builder(this)
            }

        return builder
            .setContentTitle("Sensor Studio Core")
            .setContentText("Core service is running")
            .setSmallIcon(android.R.drawable.stat_notify_sync)
            .setOngoing(true)
            .build()
    }

    /**
     * assets/config 전체를 filesDir/config로 복사하고,
     * runtime.toml의 Android 네이티브 라이브러리 경로를 치환한다.
     */
    private fun prepareRuntimeConfig() {
        val configDirectory = File(filesDir, "config")

        copyAssetDirectory(
            assetPath = "config",
            destination = configDirectory
        )

        val runtimeConfig = File(
            configDirectory,
            "runtime.toml"
        )

        if (!runtimeConfig.isFile) {
            throw IllegalStateException(
                "runtime.toml not found: ${runtimeConfig.absolutePath}"
            )
        }

        val originalConfig = runtimeConfig.readText(Charsets.UTF_8)

        if (!originalConfig.contains("__NATIVE_LIBRARY_DIR__")) {
            Log.w(
                TAG,
                "__NATIVE_LIBRARY_DIR__ placeholder was not found"
            )
        }

        val resolvedConfig = originalConfig.replace(
            "__NATIVE_LIBRARY_DIR__",
            applicationInfo.nativeLibraryDir
        )

        runtimeConfig.writeText(
            resolvedConfig,
            Charsets.UTF_8
        )

        Log.i(
            TAG,
            "Runtime config prepared: ${runtimeConfig.absolutePath}"
        )
    }

    /**
     * assets 안의 디렉터리와 파일을 재귀적으로 복사한다.
     */
    private fun copyAssetDirectory(
        assetPath: String,
        destination: File
    ) {
        val children = assets.list(assetPath)
            ?: throw IllegalStateException(
                "Failed to list asset path: $assetPath"
            )

        if (children.isEmpty()) {
            destination.parentFile?.let { parent ->
                if (!parent.exists() && !parent.mkdirs()) {
                    throw IllegalStateException(
                        "Failed to create directory: ${parent.absolutePath}"
                    )
                }
            }

            assets.open(assetPath).use { input ->
                destination.outputStream().use { output ->
                    input.copyTo(output)
                }
            }

            return
        }

        if (!destination.exists() && !destination.mkdirs()) {
            throw IllegalStateException(
                "Failed to create directory: ${destination.absolutePath}"
            )
        }

        children.forEach { child ->
            copyAssetDirectory(
                assetPath = "$assetPath/$child",
                destination = File(destination, child)
            )
        }
    }
}