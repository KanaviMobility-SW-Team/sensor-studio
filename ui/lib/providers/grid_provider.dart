import 'package:riverpod_annotation/riverpod_annotation.dart';

import 'package:ui/providers/settings_storage_provider.dart';

part 'grid_provider.g.dart';

class GridSettings {
  final double gridSize;
  final double gridStep;
  final bool showGrid;
  final bool showGridAxis;
  final bool showGridLabels;

  const GridSettings({
    this.gridSize = 30,
    this.gridStep = 1,
    this.showGrid = true,
    this.showGridAxis = true,
    this.showGridLabels = true,
  });

  factory GridSettings.fromJson(Map<String, dynamic> json) {
    return GridSettings(
      gridSize: (json['gridSize'] as num?)?.toDouble() ?? 30.0,
      gridStep: (json['gridStep'] as num?)?.toDouble() ?? 1.0,
      showGrid: json['showGrid'] as bool? ?? true,
      showGridAxis: json['showGridAxis'] as bool? ?? true,
      showGridLabels: json['showGridLabels'] as bool? ?? true,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'showGrid': showGrid,
      'gridSize': gridSize,
      'gridStep': gridStep,
      'showGridLabels': showGridLabels,
      'showGridAxis': showGridAxis,
    };
  }

  GridSettings copyWith({
    double? gridSize,
    double? gridStep,
    bool? showGrid,
    bool? showGridAxis,
    bool? showGridLabels,
  }) => GridSettings(
    gridSize: gridSize ?? this.gridSize,
    gridStep: gridStep ?? this.gridStep,
    showGrid: showGrid ?? this.showGrid,
    showGridAxis: showGridAxis ?? this.showGridAxis,
    showGridLabels: showGridLabels ?? this.showGridLabels,
  );
}

@riverpod
class GridSettingsNotifier extends _$GridSettingsNotifier {
  static const _fileName = 'grid_settings.json';

  @override
  GridSettings build() {
    Future.microtask(_load);
    return const GridSettings();
  }

  Future<void> _load() async {
    final storage = ref.read(settingsFileStorageProvider);
    final json = await storage.readJson(_fileName);

    if (json == null) {
      return;
    }

    state = GridSettings.fromJson(json);
  }

  Future<void> _save() async {
    final storage = ref.read(settingsFileStorageProvider);
    await storage.writeJson(_fileName, state.toJson());
  }

  Future<void> updateGridSize(double value) async {
    state = state.copyWith(gridSize: value);
    await _save();
  }

  Future<void> updateGridStep(double value) async {
    state = state.copyWith(gridStep: value);
    await _save();
  }

  Future<void> updateShowGrid(bool value) async {
    state = state.copyWith(showGrid: value);
    await _save();
  }

  Future<void> updateShowGridAxis(bool value) async {
    state = state.copyWith(showGridAxis: value);
    await _save();
  }

  Future<void> updateShowGridLabels(bool value) async {
    state = state.copyWith(showGridLabels: value);
    await _save();
  }
}
