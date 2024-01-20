
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:localstorage/localstorage.dart';

class Callback extends StatelessWidget {
  const Callback({super.key});

  @override
  Widget build(BuildContext context) {
    String jobID = Uri.parse(Uri.base.toString().replaceAll("#/", "")).queryParameters["jobId"].toString();
    late dynamic jobJSON;
    late LocalStorage localStore;
    http.get(Uri.parse("http://localhost:8642/getjob/$jobID")).then((resp) => {
      jobJSON = jsonDecode(resp.body),
      localStore = LocalStorage("currentSession"),
      localStore.ready.then((_) => {
        localStore.setItem("jobID", jobID),
        localStore.setItem("connectionID", jobJSON["response_data"]["payload"]["connectionID"].toString())
      })
    });
    context.go('/dashboard');

    return const MaterialApp(
      title: "Redirecting to dashboard",
    );
  }

}