import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
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
