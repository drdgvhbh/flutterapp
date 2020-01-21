import 'package:documentation_app/providers.dart';
import 'package:documentation_app/repository.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'editor_page.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();

  runApp(MultiProvider(
    providers: [
      linkProvider,
      cacheProvider,
      graphQLClientProvider,
      databaseProvider,
      documentRepositoryProvider
    ],
    child: App(),
  ));
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Quick Start',
      home: HomePage(),
      routes: {
        "/editor": (context) => Consumer<DocumentRepository>(
              builder: (_, documentRepository, child) {
                return EditorPage(
                  documentRepository: documentRepository,
                );
              },
            )
        // result
      },
    );
  }
}

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final navigator = Navigator.of(context);
    return Scaffold(
      appBar: AppBar(title: Text("Quick Start")),
      body: Center(
        child: FlatButton(
          child: Text("Open editor"),
          onPressed: () => navigator.pushNamed("/editor"),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => navigator.pushNamed("/editor"),
        child: Icon(Icons.create),
        backgroundColor: Colors.blueAccent,
      ),
    );
  }
}
