// app.dart
import 'package:flutter/material.dart';
import 'package:sensor_studio/features/sensor/widgets/sensor_setting_panel.dart';
import 'core/theme/app_theme.dart';
import 'shared/widgets/app_scaffold.dart';
import 'shared/widgets/app_toolbar.dart';
import 'features/sensor/widgets/sensor_list_panel.dart';
import 'features/viewer/pointcloud_viewer.dart';

class SensorStudioApp extends StatelessWidget {
  const SensorStudioApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Sensor Studio',
      theme: AppTheme.dark(),
      home: const MainScreen(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class MainScreen extends StatelessWidget {
  const MainScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return AppScaffold(
      toolbar: const AppToolbar(),
      left: const SensorListPanel(),
      center: const PointCloudViewer(),
      right: const SensorSettingPanel(),
    );
  }
}
