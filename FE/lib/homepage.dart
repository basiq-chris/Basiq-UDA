

import 'package:flutter/material.dart';
import 'package:localstorage/localstorage.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<StatefulWidget> createState() => HomePageState();

}

class HomePageState extends State<HomePage> {

  @override
  Widget build(BuildContext context) {
    String jobID;
    LocalStorage localStorage = LocalStorage("LocalStorage.json");
    localStorage.ready.then((_) => {
      jobID = localStorage.getItem("jobID"),
    });

    return const MaterialApp(
      title: "SallyAX Homepage",

      home: Text("WORK IN PROGRESS"),
    );
  }

}