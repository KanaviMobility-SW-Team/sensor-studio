import 'package:riverpod_annotation/riverpod_annotation.dart';

import 'package:ui/providers/settings_storage_provider.dart';

part 'ui_layout_provider.g.dart';

class UILayoutState {
  final bool isLeftSidebarVisible;
  final bool isRightSidebarVisible;
  final bool isBottombarVisible;

  UILayoutState({
    required this.isLeftSidebarVisible,
    required this.isRightSidebarVisible,
    required this.isBottombarVisible,
  });

  factory UILayoutState.fromJson(Map<String, dynamic> json) {
    return UILayoutState(
      isLeftSidebarVisible: json['isLeftSidebarVisible'] as bool? ?? true,
      isRightSidebarVisible: json['isRightSidebarVisible'] as bool? ?? false,
      isBottombarVisible: json['isBottombarVisible'] as bool? ?? true,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'isLeftSidebarVisible': isLeftSidebarVisible,
      'isRightSidebarVisible': isRightSidebarVisible,
      'isBottombarVisible': isBottombarVisible,
    };
  }
}

@riverpod
class UILayout extends _$UILayout {
  static const _fileName = 'ui_layout.json';

  @override
  UILayoutState build() {
    Future.microtask(_load);
    return UILayoutState(
      isLeftSidebarVisible: true,
      isRightSidebarVisible: false,
      isBottombarVisible: true,
    );
  }

  Future<void> _load() async {
    final storage = ref.read(settingsFileStorageProvider);
    final json = await storage.readJson(_fileName);

    if (json == null) {
      return;
    }

    state = UILayoutState.fromJson(json);
  }

  Future<void> _save() async {
    final storage = ref.read(settingsFileStorageProvider);
    await storage.writeJson(_fileName, state.toJson());
  }

  void updateLeftSidebarVisibility(bool isVisible) {
    state = UILayoutState(
      isLeftSidebarVisible: isVisible,
      isRightSidebarVisible: state.isRightSidebarVisible,
      isBottombarVisible: state.isBottombarVisible,
    );
    _save();
  }

  void updateRightSidebarVisibility(bool isVisible) {
    state = UILayoutState(
      isLeftSidebarVisible: state.isLeftSidebarVisible,
      isRightSidebarVisible: isVisible,
      isBottombarVisible: state.isBottombarVisible,
    );
    _save();
  }

  void updateBottombarVisibility(bool isVisible) {
    state = UILayoutState(
      isLeftSidebarVisible: state.isLeftSidebarVisible,
      isRightSidebarVisible: state.isRightSidebarVisible,
      isBottombarVisible: isVisible,
    );
    _save();
  }
}
