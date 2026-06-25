import 'package:flutter/material.dart';

import 'package:ui/widgets/overlays/loading_overlay.dart';
import 'package:ui/widgets/layout/main_header.dart';
import 'package:ui/widgets/layout/sensor_sidebar.dart';
import 'package:ui/widgets/layout/bottom_console.dart';
import 'package:ui/widgets/visualizer/main_visualizer.dart';

class DashboardScreen extends StatelessWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: LoadingOverlay(
        child: Column(
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
      ),
    );
  }
}
