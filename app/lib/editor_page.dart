import 'package:documentation_app/repository.dart';
import 'package:flutter/material.dart';
import 'package:quill_delta/quill_delta.dart';
import 'package:zefyr/zefyr.dart';
import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';

/* typedef void OnChange(String change);

OnChange onChangeDefaultCallback(String s) {} */

class EditorPage extends StatefulWidget {
  const EditorPage({
    @required this.documentRepository,
  });

  final DocumentRepository documentRepository;

  @override
  EditorPageState createState() => EditorPageState();
}

class EditorPageState extends State<EditorPage> {
  /// Allows to control the editor and the document.
  ZefyrController _controller;

  TextEditingController _titleController;

  FocusNode _titleFocusNode;

  /// Zefyr editor like any other input field requires a focus node.
  FocusNode _focusNode;

  String _id;

  @override
  void initState() {
    super.initState();
    _focusNode = FocusNode();
    _titleFocusNode = FocusNode();
    _titleController = TextEditingController();
    _id = "2bd0becf-c8a0-4fbe-aad3-6d0ff919084c";
    _loadDocument().then((document) {
      setState(() {
        _controller = ZefyrController(document);
        document.changes.listen((change) {
          if (change.source != ChangeSource.local) {
            return;
          }
          final delta = jsonEncode(change.change.toJson());
          debugPrint("delta" + delta);

          print("sadasd " + jsonEncode(change.before.toJson()));
          final parentBytes = utf8.encode(jsonEncode(change.before.toJson()));
          final parentHash = sha256.convert(parentBytes);
          debugPrint("serialized " + parentHash.toString());

          final result = this
              .widget
              .documentRepository
              .updateDocument(_id, delta, parentHash.toString());

/*           result.then((val) {
            debugPrint(val.data.toString());
            debugPrint(val.exception.toString());
          }).catchError((err) => debugPrint("err" + err.toString()));
 */
          // change.before.concat(other);
          // change.change.concat(other)
          // debugPrint(change.source.toString());
          // debugPrint(jsonEncode(change.change.toJson()));
          String plaintext = document.lookupLine(0).node.toPlainText();
          String firstLine = plaintext.substring(0, plaintext.indexOf("\n"));
          // document.format(0, firstLine.length, NotusAttribute.h1);
        });
      });
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    _titleController.dispose();
    _focusNode.dispose();
    _titleFocusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = ZefyrThemeData();
    // If _controller is null we show Material Design loader, otherwise
    // display Zefyr editor.
    final Widget body = (_controller == null)
        ? Center(child: CircularProgressIndicator())
        : ZefyrScaffold(
            child: ZefyrTheme(
              data: theme,
              child: ZefyrEditor(
                padding: EdgeInsets.all(16),
                controller: _controller,
                focusNode: _focusNode,
              ),
            ),
          );

    return Scaffold(
      appBar: AppBar(
        actions: <Widget>[
          Builder(
            builder: (context) => IconButton(
              icon: Icon(Icons.save),
              onPressed: () => _saveDocument(context),
            ),
          )
        ],
      ),
      body: body,
    );
  }

  /// Loads the document to be edited in Zefyr.
  Future<NotusDocument> _loadDocument() async {
    final result = await widget.documentRepository.getDocument(_id);
    if (result.exception != null) {
      throw result.exception;
    }

    final derp = result.data as Map<dynamic, dynamic>;
    debugPrint("aaa" + derp.toString());
    final derp2 = jsonDecode(derp["document"]) as List<dynamic>;

    debugPrint("aaa" + derp2.toString());

    final zeDeltas = Delta.fromJson(derp2);

    debugPrint("zeDeltas" + zeDeltas.toString());

    // debugPrint(NotusDocument.fromDelta(zeDeltas).toString());
    debugPrint(NotusDocument().toDelta().toJson().toString());

    return NotusDocument();
  }

  void _saveDocument(BuildContext context) {
    // Notus documents can be easily serialized to JSON by passing to
    // `jsonEncode` directly
    final contents = jsonEncode(_controller.document);
    // For this example we save our document to a temporary file.
    final file = File(Directory.systemTemp.path + "/quick_start.json");
    // And show a snack bar on success.
    file.writeAsString(contents).then((_) {
      Scaffold.of(context).showSnackBar(SnackBar(content: Text("Saved.")));
    });
  }
}
