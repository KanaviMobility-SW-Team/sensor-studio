// shared/widgets/app_scaffold.dart
import 'package:flutter/material.dart';

class AppScaffold extends StatelessWidget {
  final PreferredSizeWidget toolbar;
  final Widget left;
  final Widget center;

  const AppScaffold({
    super.key,
    required this.toolbar,
    required this.left,
    required this.center,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: toolbar,
      body: Row(
        children: [
          SizedBox(width: 280, child: left),
          const VerticalDivider(width: 1),
          Expanded(child: center),
        ],
      ),
    );
  }
}
