import 'package:flutter/material.dart';

class AppScaffold extends StatefulWidget {
  final PreferredSizeWidget toolbar;
  final Widget left;
  final Widget center;
  final Widget right;

  const AppScaffold({
    super.key,
    required this.toolbar,
    required this.left,
    required this.center,
    required this.right,
  });

  @override
  State<AppScaffold> createState() => _AppScaffoldState();
}

class _AppScaffoldState extends State<AppScaffold> {
  bool _leftOpen = true;
  bool _rightOpen = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: widget.toolbar,
      body: Row(
        children: [
          // Left panel
          _leftOpen
              ? SizedBox(
                  width: 280,
                  child: Stack(
                    children: [
                      widget.left,
                      Positioned(
                        right: 0,
                        top: 0,
                        child: IconButton(
                          icon: Icon(Icons.chevron_left),
                          onPressed: () => setState(() => _leftOpen = false),
                        ),
                      ),
                    ],
                  ),
                )
              : IconButton(
                  icon: Icon(Icons.chevron_right),
                  onPressed: () => setState(() => _leftOpen = true),
                ),
          if (_leftOpen) const VerticalDivider(width: 1),
          Expanded(child: widget.center),
          if (_rightOpen) const VerticalDivider(width: 1),
          // Right panel
          _rightOpen
              ? SizedBox(
                  width: 330,
                  child: Stack(
                    children: [
                      widget.right,
                      Positioned(
                        left: 0,
                        top: 0,
                        child: IconButton(
                          icon: Icon(Icons.chevron_right),
                          onPressed: () => setState(() => _rightOpen = false),
                        ),
                      ),
                    ],
                  ),
                )
              : IconButton(
                  icon: Icon(Icons.chevron_left),
                  onPressed: () => setState(() => _rightOpen = true),
                ),
        ],
      ),
    );
  }
}
// ...existing code...