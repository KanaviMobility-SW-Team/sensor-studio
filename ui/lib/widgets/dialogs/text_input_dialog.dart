import 'package:flutter/material.dart';

import 'package:ui/theme/app_colors.dart';

class TextInputDialog extends StatefulWidget {
  final String title;
  final String hintText;
  final String initialValue;
  final String submitButtonText;
  final String cancelButtonText;
  final Function(String) onSubmitted;

  const TextInputDialog({
    super.key,
    required this.title,
    required this.hintText,
    this.initialValue = '',
    this.submitButtonText = 'Submit',
    this.cancelButtonText = 'Cancel',
    required this.onSubmitted,
  });

  @override
  State<TextInputDialog> createState() => _TextInputDialogState();
}

class _TextInputDialogState extends State<TextInputDialog> {
  late TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.initialValue);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(widget.title),
      content: TextSelectionTheme(
        data: TextSelectionThemeData(
          cursorColor: AppColors.accent,
          selectionColor: AppColors.accent.withAlpha(100),
          selectionHandleColor: AppColors.accent,
        ),
        child: TextField(
          cursorColor: AppColors.accent,
          controller: _controller,
          decoration: InputDecoration(
            hintText: widget.hintText,
            focusedBorder: UnderlineInputBorder(
              borderSide: BorderSide(color: AppColors.accent, width: 1.5),
            ),
          ),
          autofocus: true,
          onSubmitted: (value) {
            widget.onSubmitted(value.trim());
            Navigator.of(context).pop();
          },
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          style: TextButton.styleFrom(foregroundColor: Colors.white70),
          child: Text(widget.cancelButtonText),
        ),
        TextButton(
          onPressed: () {
            widget.onSubmitted(_controller.text.trim());
            Navigator.of(context).pop();
          },
          style: TextButton.styleFrom(foregroundColor: AppColors.accent),
          child: Text(widget.submitButtonText),
        ),
      ],
    );
  }
}

Future<String> textInputShowDialog({
  required BuildContext context,
  String title = '',
  String hint = '',
  String init = '',
  String submitText = 'Submit',
  String cancelText = 'Cancel',
}) async {
  String value = '';

  await showDialog(
    context: context,
    builder: (context) {
      return TextInputDialog(
        title: title,
        hintText: hint,
        initialValue: init,
        submitButtonText: submitText,
        cancelButtonText: cancelText,
        onSubmitted: (inputValue) => value = inputValue,
      );
    },
  );

  return value;
}
