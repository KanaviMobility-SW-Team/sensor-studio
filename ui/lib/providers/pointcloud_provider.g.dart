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
    extends $NotifierProvider<PointCloudData, Map<String, PointCloudBuffer>> {
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
  Override overrideWithValue(Map<String, PointCloudBuffer> value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<Map<String, PointCloudBuffer>>(
        value,
      ),
    );
  }
}

String _$pointCloudDataHash() => r'b446d1ca2b9ed89b67276ba1e18c63cbca2de9fa';

abstract class _$PointCloudData
    extends $Notifier<Map<String, PointCloudBuffer>> {
  Map<String, PointCloudBuffer> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref
            as $Ref<
              Map<String, PointCloudBuffer>,
              Map<String, PointCloudBuffer>
            >;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<
                Map<String, PointCloudBuffer>,
                Map<String, PointCloudBuffer>
              >,
              Map<String, PointCloudBuffer>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
