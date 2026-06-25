// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sensor_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(SensorList)
final sensorListProvider = SensorListProvider._();

final class SensorListProvider
    extends $NotifierProvider<SensorList, List<SensorConfig>> {
  SensorListProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'sensorListProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$sensorListHash();

  @$internal
  @override
  SensorList create() => SensorList();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(List<SensorConfig> value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<List<SensorConfig>>(value),
    );
  }
}

String _$sensorListHash() => r'765d0ed91b1fe411a73501eec25721cf6c072de7';

abstract class _$SensorList extends $Notifier<List<SensorConfig>> {
  List<SensorConfig> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<List<SensorConfig>, List<SensorConfig>>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<List<SensorConfig>, List<SensorConfig>>,
              List<SensorConfig>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
