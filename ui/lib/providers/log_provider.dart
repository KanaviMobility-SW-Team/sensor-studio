import 'package:logging/logging.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'log_provider.g.dart';

// UI에 보여줄 최대 로그 줄 수
const int maxLogLines = 50;

@riverpod
class SystemLogs extends _$SystemLogs {
  @override
  List<String> build() {
    // 앱 전역의 로거 이벤트를 수신하여 Riverpod 상태(state)에 추가합니다.
    Logger.root.onRecord.listen((LogRecord record) {
      _addLog(
        '[${record.level.name}] ${record.loggerName} - ${record.message}',
      );
    });

    return ['[INFO] System Logger Initialized.'];
  }

  void _addLog(String message) {
    final time = DateTime.now().toIso8601String().substring(11, 19);
    final newLog = '[$time] $message';

    // 로그가 너무 많아지면 오래된 것을 지웁니다.
    if (state.length >= maxLogLines) {
      state = [...state.sublist(1), newLog];
    } else {
      state = [...state, newLog];
    }
  }

  // 필요한 경우 수동으로 화면 로그를 지우는 기능
  void clearLogs() {
    state = [];
  }
}
