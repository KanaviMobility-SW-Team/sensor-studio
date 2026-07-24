cd ../core

bash ./build.sh

cd ../SensorStudioCoreService

cp \
  ../core/dist/*.so \
  ./app/src/main/jniLibs/arm64-v8a/libsensor_studio_core.so