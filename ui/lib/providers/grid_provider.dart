import 'package:flutter_riverpod/flutter_riverpod.dart';

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

class GridSettingsNotifier extends Notifier<GridSettings> {
  @override
  GridSettings build() => const GridSettings();

  void updateGridSize(double value) => state = state.copyWith(gridSize: value);
  void updateGridStep(double value) => state = state.copyWith(gridStep: value);
  void updateShowGrid(bool value) => state = state.copyWith(showGrid: value);
  void updateShowGridAxis(bool value) =>
      state = state.copyWith(showGridAxis: value);
  void updateShowGridLabels(bool value) =>
      state = state.copyWith(showGridLabels: value);
}

final gridSettingsProvider =
    NotifierProvider<GridSettingsNotifier, GridSettings>(
      GridSettingsNotifier.new,
    );
