import 'dart:io';
import 'dart:isolate';

import 'package:osrv/osrv.dart' show Server;
import 'package:osrv/runtime/dart.dart' show serve;
import 'package:spry/spry.dart';

Future<void> runServer([_]) async {
  final app = Spry(
    routes: {
      '/': {HttpMethod.get: (_) => Response(status: 200)},
      '/user': {HttpMethod.post: (_) => Response(status: 200)},
      '/user/:name': {
        HttpMethod.get: (event) => Response.text(event.params.required('name')),
      },
    },
  );

  final runtime = await serve(
    Server(fetch: app.fetch),
    host: '0.0.0.0',
    port: 3000,
    shared: true,
  );

  await runtime.closed;
}

Future<void> main() async {
  // Run cluster servers.
  for (int i = Platform.numberOfProcessors - 1; i > 0; i--) {
    await Isolate.spawn(runServer, null);
  }

  await runServer();
}
