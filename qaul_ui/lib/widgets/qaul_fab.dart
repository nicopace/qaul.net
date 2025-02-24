import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class QaulFAB extends StatelessWidget {
  const QaulFAB({
    Key? key,
    required this.svgAsset,
    required this.onPressed,
    this.heroTag,
    this.tooltip,
    this.size = 52,
  }) : super(key: key);

  final String svgAsset;
  final VoidCallback onPressed;
  final String? heroTag;
  final String? tooltip;
  final int size;

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton.large(
      elevation: 0,
      heroTag: heroTag,
      backgroundColor: Colors.white,
      tooltip: tooltip,
      shape: const CircleBorder(side: BorderSide(color: Colors.black)),
      onPressed: onPressed,
      child: SvgPicture.asset(
        svgAsset,
        width: 48,
        height: 48,
        color: Colors.grey.shade600,
      ),
    );
  }
}
