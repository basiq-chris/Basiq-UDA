
import 'dart:convert';

import 'package:fe/callback.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:localstorage/localstorage.dart';

class HomePage extends StatefulWidget {
  final Job jobID;
  const HomePage({super.key, required this.jobID});

  @override
  State<StatefulWidget> createState() => HomePageState(jobID.jobID);

}

class HomePageState extends State<HomePage> {
  String jobID;
  HomePageState(this.jobID);
  @override
  Widget build(BuildContext context) {
    debugPrint(jobID);
    late dynamic jobJSON;
    late LocalStorage localStore;
    http.get(Uri.parse("http://127.0.0.1:8642/$jobID")).then((resp) => {
      jobJSON = jsonDecode(resp.body),
      localStore = LocalStorage(jobJSON["response_data"]["payload"]["userID"].toString()),
      localStore.setItem("connectionID", jobJSON["response_data"]["payload"]["connectionID"].toString())
    });

    return const MaterialApp(
      title: "SallyAX Homepage",

      home: Text("WORK IN PROGRESS"),
    );
  }

}