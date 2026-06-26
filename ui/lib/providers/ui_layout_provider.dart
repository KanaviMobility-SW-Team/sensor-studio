import 'package:riverpod_annotation/riverpod_annotation.dart';

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
}

@riverpod
class UILayout extends _$UILayout {
  @override
  UILayoutState build() => UILayoutState(
    isLeftSidebarVisible: true,
    isRightSidebarVisible: false,
    isBottombarVisible: true,
  );

  void updateLeftSidebarVisibility(bool isVisible) {
    state = UILayoutState(
      isLeftSidebarVisible: isVisible,
      isRightSidebarVisible: state.isRightSidebarVisible,
      isBottombarVisible: state.isBottombarVisible,
    );
  }

  void updateRightSidebarVisibility(bool isVisible) {
    state = UILayoutState(
      isLeftSidebarVisible: state.isLeftSidebarVisible,
      isRightSidebarVisible: isVisible,
      isBottombarVisible: state.isBottombarVisible,
    );
  }

  void updateBottombarVisibility(bool isVisible) {
    state = UILayoutState(
      isLeftSidebarVisible: state.isLeftSidebarVisible,
      isRightSidebarVisible: state.isRightSidebarVisible,
      isBottombarVisible: isVisible,
    );
  }
}
