import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass/point_glass.dart';
import 'package:vector_math/vector_math.dart' as vm;

import '../../providers/sensor_provider.dart';

class MainVisualizer extends ConsumerStatefulWidget {
  const MainVisualizer({super.key});

  @override
  ConsumerState<MainVisualizer> createState() => _MainVisualizerState();
}

class _MainVisualizerState extends ConsumerState<MainVisualizer> {
  late final ValueNotifier<ViewContext> _viewContext;

  @override
  void initState() {
    super.initState();
    _viewContext = ValueNotifier(
      ViewContext(
        model: ModelTransform(),
        camera: PinholeCamera(cameraZ: 20),
        proj: PinholeProjection(focalPx: 800, near: 1, far: 20000),
        canvasCenter: const Offset(0, 0),
      ),
    );
  }

  @override
  void dispose() {
    _viewContext.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final sensors = ref.watch(sensorListProvider);

    final pointsGroup = sensors.where((s) => s.isVisible).map((sensor) {
      // 임시 더미 데이터 생성
      List<vm.Vector3> dummyVectors;
      Color baseColor;

      if (sensor.name == 'lidar_roof') {
        dummyVectors = [
          vm.Vector3(0, 0, 5),
          vm.Vector3(1, 0, 5),
          vm.Vector3(0, 1, 5),
          vm.Vector3(1, 1, 5),
        ];
        baseColor = Colors.redAccent;
      } else if (sensor.name == 'lidar_bumper') {
        dummyVectors = [
          vm.Vector3(3, 0, 1),
          vm.Vector3(4, 0, 1),
          vm.Vector3(3, 1, 1),
          vm.Vector3(4, 1, 1),
        ];
        baseColor = Colors.blueAccent;
      } else {
        // radar_front
        dummyVectors = [
          vm.Vector3(-3, 0, 2),
          vm.Vector3(-4, 0, 2),
          vm.Vector3(-3, 1, 2),
          vm.Vector3(-4, 1, 2),
        ];
        baseColor = Colors.greenAccent;
      }

      final pointList = dummyVectors.map((v) {
        return PointGlassPoint(
          point: v,
          strokeWidth: sensor.pointSize, // 사이드바의 Point Size 슬라이더 연동
          alpha: (sensor.opacity * 255).toInt(), // 0.0~1.0 값을 0~255로 변환하여 연동
          color: baseColor,
        );
      }).toList();

      return PointGlassPoints(enable: true, points: pointList);
    }).toList();

    return Expanded(
      child: Container(
        color: const Color(0xFF121212),
        child: ClipRect(
          child: PointGlassViewer(
            viewContext: _viewContext,
            mode: PointGlassViewerMode.rotate,
            grid: PointGlassGrid(
              enable: true,
              gridSize: 20,
              gridStep: 1,
              enableLabel: true,
              labelStyle: TextStyle(color: Colors.white.withAlpha(150)),
            ),
            axis: PointGlassAxis(enable: true, axisLength: 2.0),
            // 동적으로 생성된 pointsGroup을 주입
            pointsGroup: pointsGroup,
          ),
        ),
      ),
    );
  }
}
