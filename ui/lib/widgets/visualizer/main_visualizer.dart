import 'package:flutter/material.dart';

import 'package:point_glass/point_glass.dart';
import 'package:vector_math/vector_math.dart' as vm;

class MainVisualizer extends StatefulWidget {
  const MainVisualizer({super.key});

  @override
  State<MainVisualizer> createState() => _MainVisualizerState();
}

class _MainVisualizerState extends State<MainVisualizer> {
  late final ValueNotifier<ViewContext> _viewContext;

  @override
  void initState() {
    super.initState();
    _viewContext = ValueNotifier(
      ViewContext(
        model: ModelTransform(),
        camera: PinholeCamera(cameraZ: 20), // 초기 카메라 거리 설정
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
    return Expanded(
      child: Container(
        color: const Color(0xFF121212), // 어두운 배경 유지
        // ClipRect를 추가하여 영역 밖으로 렌더링되는 것을 방지
        child: ClipRect(
          child: PointGlassViewer(
            viewContext: _viewContext,
            mode: PointGlassViewerMode.rotate, // 기본 마우스 동작: 회전
            // 바닥 격자(Grid) 설정
            grid: PointGlassGrid(
              enable: true,
              gridSize: 20,
              gridStep: 1,
              enableLabel: true,
              labelStyle: TextStyle(color: Colors.white.withAlpha(150)),
            ),

            // X, Y, Z 축 표시
            axis: PointGlassAxis(enable: true, axisLength: 2.0),

            // 테스트용 더미 포인트 클라우드 포인트
            pointsGroup: [
              PointGlassPoints(
                enable: true,
                points: [
                  PointGlassPoint(
                    point: vm.Vector3(0, 0, 0),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                  PointGlassPoint(
                    point: vm.Vector3(0, 0, 0),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                  PointGlassPoint(
                    point: vm.Vector3(1, 0, 0),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                  PointGlassPoint(
                    point: vm.Vector3(0, 1, 0),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                  PointGlassPoint(
                    point: vm.Vector3(0, 0, 1),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                  PointGlassPoint(
                    point: vm.Vector3(2, 2, 2),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                  PointGlassPoint(
                    point: vm.Vector3(-2, -2, -2),
                    strokeWidth: 2,
                    alpha: 200,
                    color: Colors.yellow,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
