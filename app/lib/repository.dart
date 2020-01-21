import 'package:documentation_app/models.dart';
import 'package:flutter/foundation.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

const String document = r'''
  query Document($id: String!) {
    document(input: { id: $id })
  }
''';

const String change = r'''
  mutation Change($change: String!, $id: String!, $parentHash: String!) {
    change(input: { id: $id, change: $change, parentHash: $parentHash })
  }
''';

class DocumentRepository {
  final GraphQLClient client;

  final Database db;

  DocumentRepository({@required this.client, @required this.db})
      : assert(client != null);

  Future<QueryResult> getDocument(String id) async {
    final options =
        QueryOptions(documentNode: gql(document), variables: <String, String>{
      'id': id,
    });

    return await client.query(options);
  }

  Future<QueryResult> updateDocument(
      String id, String delta, String parentHash) async {
/*     final options =
        MutationOptions(documentNode: gql(change), variables: <String, String>{
      "id": id,
      "change": delta,
      "parentHash": parentHash,
    });

    debugPrint(options.toString());

    return await client.mutate(options); */
    final id = Uuid().v4();

    debugPrint(db.toString());
    await db.insert(
        'document_changes',
        DocumentChange(
                id: id, documentId: id, delta: delta, parentHash: parentHash)
            .toJson());
  }
}
