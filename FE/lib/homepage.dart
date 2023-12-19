

import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:localstorage/localstorage.dart';
import 'package:http/http.dart' as http;
import 'package:url_launcher/url_launcher.dart';
import 'package:url_launcher/url_launcher_string.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<StatefulWidget> createState() => HomePageState();

}

class HomePageState extends State<HomePage> {

  @override
  Widget build(BuildContext context) {
    late String jobID;
    bool isJobDone = false;
    bool hasJobFailed = false;
    LocalStorage localStorage = LocalStorage("LocalStorage.json");
    localStorage.ready.then((_) => {
      jobID = localStorage.getItem("jobID"),
    });


    while(!isJobDone) {
      http.get(Uri.parse("https://127.0.0.1:8642/job/$jobID/poll")).then((resp) => {
        switch (resp.statusCode) {
          200 => isJobDone = true,
          424 => {isJobDone = true, hasJobFailed = true},
          int() => null,
        },
        sleep(const Duration(seconds: 5)),
      });
    }

    if (hasJobFailed) {
      return MaterialApp(
        title: "Data failed to return",
        home: Scaffold(
          backgroundColor: Colors.red,
          body: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Text("Job failed to return data!"),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  ElevatedButton(onPressed: () => {
                    launchUrlString("https://github.com/basiq-chris/SallyAX-APP/issues/new/choose")
                  }, child: const Text("Contact Support")),
                  ElevatedButton(onPressed: () => {
                    http.post(Uri.parse("http://127.0.0.1:8642/createauthlink"), body: {"userID": localStorage.getItem("currentUser").toString()}).then((resp) => {
                      launchUrl(Uri.parse(jsonDecode(resp.body)["response_data"]["payload"]["authLink"].toString()), webOnlyWindowName: "_self")
                    })
                  }, child: const Text("Retry"))
                ],
              )
            ],
          )
        ),
      );
    }

    return const MaterialApp(
      title: "SallyAX Homepage",

      home: Text("WORK IN PROGRESS"),
    );
  }

}