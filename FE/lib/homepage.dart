

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

  List<Widget> getAccounts() {
    LocalStorage localStore = LocalStorage("currentSession");
    bool jobFailed = false;
    bool jobCompleted = false;
    getJobID() => {localStore.ready.then((_) => localStore.getItem("jobID"))} as String;
    String jobID = getJobID();
    List<AccountChip> accounts = <AccountChip>[];
    while (!jobCompleted) {
      http.post(Uri.parse("http://127.0.0.1/job/$jobID/poll")).then((resp) => {
        if (resp.statusCode == 200) {
          jobCompleted = true,
      } else if (resp.statusCode == 424) {
          jobCompleted = true,
          jobFailed = true,
        }
      });
    }
    
  }

  @override
  Widget build(BuildContext context) {
    LocalStorage localStore = LocalStorage("currentSession");
    List<Widget> accounts = [];

    return MaterialApp(
      navigatorKey: DashboardContext.navKey,
      title: "Dashboard",
      home: Scaffold(
          body: ListView.builder(
            itemCount: accounts.length,
            itemBuilder: (ctx, cnt) => {
              await localStore.ready;


              //return AccountChip();
            },
          ),
      ),
    );
  }
}

class AccountChip extends StatelessWidget {
  final String balance, accountNo, accountHolder, avaliableBalance, _accountID, _bankImg;

  const AccountChip(this.balance, this.accountNo, this.accountHolder, this.avaliableBalance, this._accountID, this._bankImg, {super.key});

  String getID() {
    return _accountID;
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2.0,
      shape: const RoundedRectangleBorder(borderRadius: BorderRadius.all(Radius.circular(2.0))),
      child: Column(
        children: [
          Row(
            children: [
              ImageIcon(NetworkImage(_bankImg))
            ],
          ),
          Text(accountHolder),
          Text(accountNo),
          Row(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Column(
                children: [
                  Text(avaliableBalance),
                  Text(balance)
                ],
              )
            ],
          )
        ],
      ),
    );
  }

}

class DashboardContext {
  static GlobalKey<NavigatorState> navKey = GlobalKey<NavigatorState>();
}