import 'package:flutter/material.dart';

import 'package:ui/theme/app_colors.dart';

class NumberInputControl extends StatefulWidget {
  final String label;
  final double value;
  final ValueChanged<double> onChanged;
  final VoidCallback? onAuto;

  const NumberInputControl({
    super.key,
    required this.label,
    required this.value,
    required this.onChanged,
    this.onAuto,
  });

  @override
  State<NumberInputControl> createState() => _NumberInputControlState();
}

class _NumberInputControlState extends State<NumberInputControl> {
  late TextEditingController _controller;
  late FocusNode _focusNode;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: _format(widget.value));
    _focusNode = FocusNode();
    _focusNode.addListener(() {
      if (!_focusNode.hasFocus) {
        _commit();
      }
    });
  }

  @override
  void didUpdateWidget(NumberInputControl oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!_focusNode.hasFocus && oldWidget.value != widget.value) {
      _controller.text = _format(widget.value);
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  String _format(double v) =>
      v == v.truncateToDouble() ? v.toInt().toString() : v.toStringAsFixed(2);

  void _commit() {
    final parsed = double.tryParse(_controller.text);
    if (parsed != null) {
      widget.onChanged(parsed);
    } else {
      _controller.text = _format(widget.value);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(top: 2.0, bottom: 2.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          SizedBox(
            width: 110,
            child: Text(
              widget.label,
              style: const TextStyle(color: Colors.white70, fontSize: 13),
            ),
          ),
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(5.0),
              child: TextField(
                controller: _controller,
                focusNode: _focusNode,
                keyboardType: const TextInputType.numberWithOptions(
                  signed: true,
                  decimal: true,
                ),
                textAlign: TextAlign.center,
                style: const TextStyle(color: Colors.white, fontSize: 13),
                decoration: InputDecoration(
                  isDense: true,
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 7,
                  ),
                  enabledBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(6),
                    borderSide: const BorderSide(color: Colors.white24),
                  ),
                  focusedBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(6),
                    borderSide: const BorderSide(color: AppColors.accent),
                  ),
                ),
                onSubmitted: (_) => _commit(),
              ),
            ),
          ),
          if (widget.onAuto != null)
            SizedBox(
              width: 28,
              height: 28,
              child: IconButton(
                padding: EdgeInsets.zero,
                onPressed: widget.onAuto,
                icon: const Icon(
                  Icons.refresh,
                  size: 16,
                  color: AppColors.accent,
                ),
              ),
            ),
        ],
      ),
    );
  }
}
