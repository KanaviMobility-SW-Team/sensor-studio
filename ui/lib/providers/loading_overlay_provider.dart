import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'loading_overlay_provider.g.dart';

@riverpod
class LoadingOverlay extends _$LoadingOverlay {
  @override
  bool build() => false;

  void show() => state = true;

  void hide() => state = false;

  void setVisible(bool visible) => state = visible;
}
