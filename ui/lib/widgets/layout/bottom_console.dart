import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../providers/log_provider.dart';

class BottomConsole extends ConsumerWidget {
  const BottomConsole({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // 작성한 SystemLogs 프로바이더를 구독
    final logs = ref.watch(systemLogsProvider);

    return Container(
      height: 150,
      color: const Color(0xFF1E1E1E),
      width: double.infinity,
      padding: const EdgeInsets.all(8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Padding(
                padding: EdgeInsets.only(left: 8.0, bottom: 4.0),
                child: Text(
                  'SYSTEM LOGS',
                  style: TextStyle(
                    color: Colors.white54,
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              // 로그 지우기 버튼 추가
              InkWell(
                onTap: () {
                  ref.read(systemLogsProvider.notifier).clearLogs();
                },
                child: const Icon(
                  Icons.delete_outline,
                  size: 16,
                  color: Colors.white54,
                ),
              ),
            ],
          ),
          const Divider(color: Colors.white24, height: 1),
          Expanded(
            // 로그가 추가될 때마다 자동으로 스크롤 되도록 ListView 구성
            child: ListView.builder(
              padding: const EdgeInsets.all(8.0),
              itemCount: logs.length,
              itemBuilder: (context, index) {
                // 새로운 로그가 아래에 쌓이도록 렌더링
                return Text(
                  logs[index],
                  style: const TextStyle(
                    color: Colors.white70,
                    fontSize: 12,
                    fontFamily: 'monospace',
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
