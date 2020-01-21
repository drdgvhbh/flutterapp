import 'package:documentation_app/repository.dart';
import 'package:flutter/rendering.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:path/path.dart';
import 'package:provider/provider.dart';
import 'package:sqflite/sqflite.dart';

final Provider<Link> linkProvider = Provider<Link>(
  create: (_) => HttpLink(
    uri: 'http://localhost:8000/graphql',
  ),
);

final Provider<Cache> cacheProvider = Provider<Cache>(
  create: (_) => InMemoryCache(),
);

final ProxyProvider2<Link, Cache, GraphQLClient> graphQLClientProvider =
    ProxyProvider2<Link, Cache, GraphQLClient>(
  update: (context, link, cache, client) {
    return GraphQLClient(cache: cache, link: link);
  },
);

final FutureProvider<Database> databaseProvider = FutureProvider<Database>(
    create: (_) async =>
        openDatabase(join(await getDatabasesPath(), 'documentation_app.db'),
            onCreate: (db, version) {
          return db.execute(r'''
              CREATE TABLE document_changes(
                id TEXT PRIMARY KEY,
                document_id TEXT NOT NULL,
                delta TEXT NOT NULL,
                parent_hash TEXT NOT NULL
                timestamp TEXT NOT NULL
              );''');
        }, version: 2));

final ProxyProvider2<GraphQLClient, Database, DocumentRepository>
    documentRepositoryProvider =
    ProxyProvider2<GraphQLClient, Database, DocumentRepository>(
  update: (context, client, db, documentRepo) {
    debugPrint("client" + client.toString() + " db " + db.toString());
    return DocumentRepository(client: client, db: db);
  },
);
