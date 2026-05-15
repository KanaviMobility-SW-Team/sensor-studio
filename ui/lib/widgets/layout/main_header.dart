import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../providers/websocket_provider.dart';

class MainHeader extends ConsumerWidget {
  const MainHeader({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // 상태 구독
    final wsState = ref.watch(webSocketManagerProvider);
    final wsNotifier = ref.read(webSocketManagerProvider.notifier);

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
          const Text(
            'Sensor-Studio',
            style: TextStyle(
              fontWeight: FontWeight.bold,
              color: Colors.white,
              fontSize: 16,
            ),
          ),
          const Spacer(),
          Container(
            width: 10,
            height: 10,
            decoration: BoxDecoration(
              color: statusColor,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 8),
          Text(
            statusText,
            style: const TextStyle(color: Colors.white70, fontSize: 13),
          ),
          const SizedBox(width: 24),
          IconButton(
            icon: const Icon(Icons.play_arrow, size: 20),
            color: Colors.greenAccent,
            onPressed: () {
              // 임시로 localhost 포트 8080 연결 시도
              wsNotifier.connect('ws://localhost:8080/ws');
            },
          ),
          IconButton(
            icon: const Icon(Icons.stop, size: 20),
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
