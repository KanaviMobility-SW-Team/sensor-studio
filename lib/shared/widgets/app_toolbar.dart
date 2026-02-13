// shared/widgets/app_toolbar.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../features/streaming/streaming_providers.dart';

class AppToolbar extends ConsumerWidget implements PreferredSizeWidget {
  const AppToolbar({super.key});

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final conn = ref.watch(connectionStateProvider);

    return AppBar(
      title: const Text('Kanavi-Mobility Sensor Studio'),
      actions: [
        conn.when(
          data: (s) => Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            child: Center(child: Text(s.name)),
          ),
          loading: () => const Padding(
            padding: EdgeInsets.symmetric(horizontal: 12),
            child: Center(child: Text('connecting...')),
          ),
          error: (_, __) => const Padding(
            padding: EdgeInsets.symmetric(horizontal: 12),
            child: Center(child: Text('error')),
          ),
        ),
      ],
    );
  }
}
