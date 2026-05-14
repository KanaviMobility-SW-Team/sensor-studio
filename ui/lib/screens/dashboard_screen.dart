import 'package:flutter/material.dart';
import '../widgets/layout/main_header.dart';
import '../widgets/layout/sensor_sidebar.dart';
import '../widgets/layout/bottom_console.dart';
import '../widgets/visualizer/main_visualizer.dart';

class DashboardScreen extends StatelessWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          const MainHeader(),
          Expanded(
            child: Row(
              children: [
                const SensorSidebar(),
                Container(width: 1, color: Colors.black),
                const MainVisualizer(),
              ],
            ),
          ),
          Container(height: 1, color: Colors.black),
          const BottomConsole(),
        ],
      ),
    );
  }
}
