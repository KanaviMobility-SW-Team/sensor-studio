import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_multiplatform_logger/flutter_multiplatform_logger.dart';
import 'package:ui/screens/dashboard_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

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
      theme: ThemeData.dark(),
      home: const DashboardScreen(),
    );
  }
}
