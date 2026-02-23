// main.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_multiplatform_logger/flutter_multiplatform_logger.dart';
import 'app.dart';

void main() async {
  await FlutterMultiplatformLogger.init();
  runApp(const ProviderScope(child: SensorStudioApp()));
}
