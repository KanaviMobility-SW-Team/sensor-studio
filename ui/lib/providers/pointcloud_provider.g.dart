// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'pointcloud_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(PointCloudData)
final pointCloudDataProvider = PointCloudDataProvider._();

final class PointCloudDataProvider
    extends $NotifierProvider<PointCloudData, Map<String, List<PointData>>> {
  PointCloudDataProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'pointCloudDataProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$pointCloudDataHash();

  @$internal
  @override
  PointCloudData create() => PointCloudData();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(Map<String, List<PointData>> value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<Map<String, List<PointData>>>(value),
    );
  }
}

String _$pointCloudDataHash() => r'64a7018a1e931ef381d77ff2e6c6424e30692d82';

abstract class _$PointCloudData
    extends $Notifier<Map<String, List<PointData>>> {
  Map<String, List<PointData>> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref
            as $Ref<Map<String, List<PointData>>, Map<String, List<PointData>>>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<
                Map<String, List<PointData>>,
                Map<String, List<PointData>>
              >,
              Map<String, List<PointData>>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
