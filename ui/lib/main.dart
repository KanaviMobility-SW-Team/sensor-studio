import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_multiplatform_logger/flutter_multiplatform_logger.dart';

void main() async {
  await FlutterMultiplatformLogger.init();
  Logger('Main').info('Application started!');

  runApp(const ProviderScope(child: SensorStudioApp()));
}

class SensorStudioApp extends StatelessWidget {
  const SensorStudioApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Sensor-Studio',
      theme: ThemeData.dark(), // 기본적으로 어두운 테마 사용
      home: const Scaffold(
        body: Center(child: Text('Sensor-Studio Initialized')), // 임시 화면
      ),
    );
  }
}
