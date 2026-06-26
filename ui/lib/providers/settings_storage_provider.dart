import 'dart:convert';
import 'dart:io';

import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:path_provider/path_provider.dart';

part 'settings_storage_provider.g.dart';

class SettingsFileStorage {
  Future<File> _file(String fileName) async {
    final dir = await getApplicationSupportDirectory();
    final settingsDir = Directory('${dir.path}/settings');

    if (!await settingsDir.exists()) {
      await settingsDir.create(recursive: true);
    }

    return File('${settingsDir.path}/$fileName');
  }

  Future<Map<String, dynamic>?> readJson(String fileName) async {
    final file = await _file(fileName);

    if (!await file.exists()) {
      return null;
    }

    try {
      final text = await file.readAsString();
      return jsonDecode(text) as Map<String, dynamic>;
    } catch (_) {
      return null;
    }
  }

  Future<void> writeJson(String fileName, Map<String, dynamic> json) async {
    final file = await _file(fileName);

    const encoder = JsonEncoder.withIndent('  ');
    await file.writeAsString(encoder.convert(json));
  }
}

@Riverpod(keepAlive: true)
class SettingsFileStorageNotifier extends _$SettingsFileStorageNotifier {
  @override
  SettingsFileStorage build() => SettingsFileStorage();
}
