import 'package:flutter/material.dart';

class MainVisualizer extends StatelessWidget {
  const MainVisualizer({super.key});

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        color: const Color(0xFF121212),
        child: const Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.rotate_left, size: 64, color: Colors.white24),
              SizedBox(height: 16),
              Text(
                '3D Visualizer Area\n(point_glass will be here)',
                textAlign: TextAlign.center,
                style: TextStyle(color: Colors.white54),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
