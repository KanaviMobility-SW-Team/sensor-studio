import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass/point_glass.dart';

/// 전역 ViewContext notifier.
///
/// ValueNotifier<ViewContext> 자체를 provider에서 보관하므로
/// PointGlassViewer에 직접 전달할 수 있고, 다른 위젯에서도
/// ref.read(viewContextProvider).value = ... 로 수정 가능하다.
final viewContextProvider = Provider<ValueNotifier<ViewContext>>((ref) {
  final notifier = ValueNotifier(
    ViewContext(camera: PinholeCamera(cameraZ: 20)),
  );

  ref.onDispose(notifier.dispose);
  return notifier;
});
