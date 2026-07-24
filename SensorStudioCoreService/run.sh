./copy_jniLibs.sh

./gradlew clean assembleDebug

adb install -r \
  app/build/outputs/apk/debug/app-debug.apk

adb shell am force-stop com.example.sensorstudiocore
adb logcat -c

adb shell am start-foreground-service \
  -n com.example.sensorstudiocore/.RustForegroundService

sleep 2

adb logcat -d -v time | grep -Ei \
  'SensorStudioCore|crate::run|runtime config|engine|WebSocket|AndroidRuntime'