

import 'dart:convert';
import 'dart:io';
import 'dart:js_interop';

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

 static Future<List<Widget>> getAccounts() async {
    LocalStorage localStore = LocalStorage("currentSession");
    bool jobFailed = false;
    bool jobCompleted = false;
    await localStore.ready;
    String jobID = localStore.getItem("jobID").toString();
    String userID = localStore.getItem("currentUser").toString();
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

    //Dart's shitty syntax does not allow "non-local returns"
    if (jobFailed) {
      return [];
    }

    var resp = await http.get(Uri.parse("http://127.0.0.1:8642/user/$userID/getaccounts"));
        for(var acc in jsonDecode(resp.body)["response_data"]["payload"]["accounts"]) {
          var inst = acc["institution"].toString();
          inst = await http.get(Uri.parse("au-api.basiq.io/public/connectors?filter=connector.id.eq('$inst')")).then((value) => value.body);
          inst = jsonDecode(resp.body)["data"][0]["institution"]["logo"]["square"].toString();
          accounts.add(AccountChip(acc["balance"].toString(), acc["accountNumber"].toString(), acc["accountHolder"].toString(), acc["availableBalance"].toString(), acc["id"].toString(), inst));
        }
    return accounts;
  }

  @override
  Widget build(BuildContext context) {
    LocalStorage localStore = LocalStorage("currentSession");

    return MaterialApp(
      navigatorKey: DashboardContext.navKey,
      title: "Dashboard",
      home: Scaffold(
          body: ListView.builder(
            itemBuilder: (ctx, idx) => {

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

class AccountListBuilder extends StatelessWidget {
  const AccountListBuilder({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(future: HomePageState.getAccounts(), builder: (ctx, snap) {
      if (snap.connectionState == ConnectionState.waiting) {
        return const Center(child: CircularProgressIndicator(color: Color(0x00BD1904)));
      }
      else if (snap.connectionState == ConnectionState.)
    })
  }
  
}

class DashboardContext {
  static GlobalKey<NavigatorState> navKey = GlobalKey<NavigatorState>();
}