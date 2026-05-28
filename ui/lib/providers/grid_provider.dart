import 'package:flutter_riverpod/flutter_riverpod.dart';

class GridSettings {
  final double gridSize;
  final double gridStep;
  final bool showGrid;

  const GridSettings({
    this.gridSize = 30,
    this.gridStep = 1,
    this.showGrid = true,
  });

  GridSettings copyWith({double? gridSize, double? gridStep, bool? showGrid}) =>
      GridSettings(
        gridSize: gridSize ?? this.gridSize,
        gridStep: gridStep ?? this.gridStep,
        showGrid: showGrid ?? this.showGrid,
      );
}

class GridSettingsNotifier extends Notifier<GridSettings> {
  @override
  GridSettings build() => const GridSettings();

  void updateGridSize(double value) => state = state.copyWith(gridSize: value);
  void updateGridStep(double value) => state = state.copyWith(gridStep: value);
  void updateShowGrid(bool value) => state = state.copyWith(showGrid: value);
}

final gridSettingsProvider =
    NotifierProvider<GridSettingsNotifier, GridSettings>(
      GridSettingsNotifier.new,
    );
