import 'package:flutter/cupertino.dart';
import 'package:json_annotation/json_annotation.dart';

part 'models.g.dart';

@JsonSerializable(fieldRename: FieldRename.snake)
class DocumentChange {
  final String id;

  final String documentId;

  final String delta;

  final String parentHash;

  final DateTime timestamp;

  DocumentChange(
      {@required this.id,
      @required this.documentId,
      @required this.delta,
      @required this.parentHash,
      @required this.timestamp});

  factory DocumentChange.fromJson(Map<String, dynamic> json) =>
      _$DocumentChangeFromJson(json);
  Map<String, dynamic> toJson() => _$DocumentChangeToJson(this);
}
