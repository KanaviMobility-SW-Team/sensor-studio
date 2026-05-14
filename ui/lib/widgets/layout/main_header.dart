import 'package:flutter/material.dart';

class MainHeader extends StatelessWidget {
  const MainHeader({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 50,
      color: const Color(0xFF252526),
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: [
          const Text(
            'Sensor-Studio',
            style: TextStyle(
              fontWeight: FontWeight.bold,
              color: Colors.white,
              fontSize: 16,
            ),
          ),
          const Spacer(),
          Container(
            width: 10,
            height: 10,
            decoration: const BoxDecoration(
              color: Colors.redAccent,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 8),
          const Text(
            'Disconnected',
            style: TextStyle(color: Colors.white70, fontSize: 13),
          ),
          const SizedBox(width: 24),
          IconButton(
            icon: const Icon(Icons.play_arrow, size: 20),
            onPressed: () {},
            color: Colors.greenAccent,
          ),
          IconButton(
            icon: const Icon(Icons.stop, size: 20),
            onPressed: () {},
            color: Colors.redAccent,
          ),
        ],
      ),
    );
  }
}
