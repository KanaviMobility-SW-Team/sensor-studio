import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:ui/providers/ui_layout_provider.dart';
import 'package:ui/widgets/overlays/loading_overlay.dart';
import 'package:ui/widgets/layout/main_header.dart';
import 'package:ui/widgets/layout/sensor_sidebar.dart';
import 'package:ui/widgets/layout/bottom_console.dart';
import 'package:ui/widgets/visualizer/main_visualizer.dart';

class DashboardScreen extends ConsumerWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final uiLayoutState = ref.watch(uILayoutProvider);

    return Scaffold(
      body: LoadingOverlay(
        child: Column(
          children: [
            const MainHeader(),
            Expanded(
              child: Row(
                children: [
                  uiLayoutState.isLeftSidebarVisible
                      ? const SensorSidebar()
                      : SizedBox.shrink(),
                  Container(width: 1, color: Colors.black),
                  const MainVisualizer(),
                ],
              ),
            ),
            Container(height: 1, color: Colors.black),
            uiLayoutState.isBottombarVisible
                ? const BottomConsole()
                : SizedBox.shrink(),
          ],
        ),
      ),
    );
  }
}
