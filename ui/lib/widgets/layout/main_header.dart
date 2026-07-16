import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:ui/providers/loading_overlay_provider.dart';
import 'package:ui/providers/websocket_provider.dart';
import 'package:ui/providers/ui_layout_provider.dart';
import 'package:ui/theme/app_colors.dart';
import 'package:ui/widgets/dialogs/text_input_dialog.dart';

class MainHeader extends ConsumerWidget {
  const MainHeader({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // 상태 구독
    final wsState = ref.watch(webSocketManagerProvider);
    final wsNotifier = ref.read(webSocketManagerProvider.notifier);
    final uiLayoutState = ref.watch(uILayoutProvider);

    ref.listen<ConnectionStatus>(
      webSocketManagerProvider.select((state) => state.status),
      (previous, next) {
        if (previous == next) {
          return;
        }

        final loadingNotifier = ref.read(loadingOverlayProvider.notifier);
        if (next == ConnectionStatus.connecting) {
          loadingNotifier.show();
        } else {
          loadingNotifier.hide();
        }
      },
    );

    // 상태에 따른 UI 색상/텍스트 분기 처리
    Color statusColor;
    String statusText;

    switch (wsState.status) {
      case ConnectionStatus.connected:
        statusColor = Colors.greenAccent;
        statusText = 'Connected';
        break;
      case ConnectionStatus.connecting:
        statusColor = Colors.orangeAccent;
        statusText = 'Connecting...';
        break;
      case ConnectionStatus.error:
        statusColor = Colors.red;
        statusText = 'Error';
        break;
      default:
        statusColor = Colors.redAccent;
        statusText = 'Disconnected';
    }

    return Container(
      height: 50,
      color: const Color(0xFF252526),
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: [
          Image.asset('assets/logo.png', height: 30),
          const SizedBox(width: 8),
          Text(
            'Sensor-Studio',
            style: TextStyle(
              fontWeight: FontWeight.bold,
              color: AppColors.accent,
              fontSize: 18,
            ),
          ),
          const SizedBox(width: 20),
          Material(
            child: InkWell(
              borderRadius: BorderRadius.circular(5),
              mouseCursor: SystemMouseCursors.click,
              hoverColor: Colors.white.withAlpha(50),
              splashColor: Colors.white.withAlpha(50),
              highlightColor: Colors.white.withAlpha(25),
              child: Image.asset(
                uiLayoutState.isLeftSidebarVisible
                    ? 'assets/sidebar_left_on.png'
                    : 'assets/sidebar_left_off.png',
                height: 20,
              ),
              onTap: () {
                ref
                    .read(uILayoutProvider.notifier)
                    .updateLeftSidebarVisibility(
                      !uiLayoutState.isLeftSidebarVisible,
                    );
              },
            ),
          ),
          // TODO: 우측 설정창 구현 후 주석 제거
          // const SizedBox(width: 8),
          // InkWell(
          //   child: Image.asset(
          //     uiLayoutState.isRightSidebarVisible
          //         ? 'assets/sidebar_right_on.png'
          //         : 'assets/sidebar_right_off.png',
          //     height: 20,
          //   ),
          //   onTap: () {
          //     ref
          //         .read(uILayoutProvider.notifier)
          //         .updateRightSidebarVisibility(
          //           !uiLayoutState.isRightSidebarVisible,
          //         );
          //   },
          // ),
          const SizedBox(width: 8),
          Material(
            child: InkWell(
              borderRadius: BorderRadius.circular(5),
              mouseCursor: SystemMouseCursors.click,
              hoverColor: Colors.white.withAlpha(50),
              splashColor: Colors.white.withAlpha(50),
              highlightColor: Colors.white.withAlpha(25),
              child: Image.asset(
                uiLayoutState.isBottombarVisible
                    ? 'assets/bottombar_on.png'
                    : 'assets/bottombar_off.png',
                height: 20,
              ),
              onTap: () {
                ref
                    .read(uILayoutProvider.notifier)
                    .updateBottombarVisibility(
                      !uiLayoutState.isBottombarVisible,
                    );
              },
            ),
          ),
          const SizedBox(width: 4),
          InkWell(
            borderRadius: BorderRadius.circular(5),
            mouseCursor: SystemMouseCursors.click,
            hoverColor: Colors.white.withAlpha(50),
            splashColor: Colors.white.withAlpha(50),
            highlightColor: Colors.white.withAlpha(25),
            child: Icon(
              Icons.mouse,
              size: 25,
              color: uiLayoutState.isMouseCoordinateEnabled
                  ? AppColors.accent.withAlpha(150)
                  : Colors.white.withAlpha(100),
            ),
            onTap: () {
              ref
                  .read(uILayoutProvider.notifier)
                  .updateMouseCoordinateVisibility(
                    !uiLayoutState.isMouseCoordinateEnabled,
                  );
            },
          ),
          const Spacer(),
          Container(
            width: 14,
            height: 14,
            decoration: BoxDecoration(
              color: statusColor,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 8),
          Text(
            statusText,
            style: const TextStyle(color: Colors.white70, fontSize: 14),
          ),
          const SizedBox(width: 8),
          wsState.status != ConnectionStatus.connected
              ? IconButton(
                  icon: const Icon(Icons.link, size: 24),
                  color: Colors.greenAccent,
                  onPressed: () async {
                    var value = await textInputShowDialog(
                      context: context,
                      title: 'Connect to WebSocket',
                      hint: 'Enterr WebSocket URL',
                      init: wsNotifier.currentAddress,
                      submitText: 'Connect',
                      cancelText: 'Cancel',
                    );

                    if (value.isEmpty) {
                      return;
                    }

                    await wsNotifier.connect(value);
                  },
                )
              : IconButton(
                  icon: const Icon(Icons.link_off, size: 20),
                  color: Colors.redAccent,
                  onPressed: () {
                    wsNotifier.disconnect();
                  },
                ),
        ],
      ),
    );
  }
}
