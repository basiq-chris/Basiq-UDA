import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:go_router/go_router.dart';
import 'package:localstorage/localstorage.dart';
import 'package:http/http.dart' as http;

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<StatefulWidget> createState() => HomePageState();
}

class HomePageState extends State<HomePage> {
  static Future<List<Widget>> getAccounts() async {
    LocalStorage localStore = LocalStorage("currentSession");
    await localStore.ready;
    String jobID = localStore.getItem("jobID").toString();
    String userID = localStore.getItem("currentUser").toString();
    List<AccountChip> accounts = <AccountChip>[];
    Future<int> jobStatus() async {
      return (await http.get(Uri.parse("http://localhost:8642/poll/$jobID")))
          .statusCode;
    }

    Future<void> checkJob() async {
      switch (await jobStatus()) {
        case 424:
          return Future.error("Error on the backend");
        case 102:
          Future.delayed(const Duration(seconds: 5));
          await checkJob();
      }
    }

    await checkJob();

    var resp = await http
        .get(Uri.parse("http://localhost:8642/user/$userID/getaccounts"));
    for (var acc in jsonDecode(resp.body)["response_data"]["payload"]
        ["account"]) {
      var inst = acc["institution"].toString();
      inst = await http
          .get(Uri.parse("http://localhost:8642/instimg/$inst"))
          .then((value) => value.body);
      accounts.add(AccountChip(
          acc["balance"].toString(),
          acc["accountNumber"].toString(),
          acc["accountHolder"].toString(),
          acc["availableBalance"].toString(),
          acc["id"].toString(),
          inst));
    }
    return accounts;
  }

  @override
  Widget build(BuildContext context) {
    LocalStorage localStore = LocalStorage("currentSession");

    return MaterialApp(
      navigatorKey: DashboardContext.navKey,
      title: "Dashboard",
      home: const Scaffold(body: AccountListBuilder()),
    );
  }
}

class AccountChip extends StatelessWidget {
  final String balance,
      accountNo,
      accountHolder,
      avaliableBalance,
      _accountID,
      _bankImg;

  const AccountChip(this.balance, this.accountNo, this.accountHolder,
      this.avaliableBalance, this._accountID, this._bankImg,
      {super.key});

  String getID() {
    return _accountID;
  }

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
      onTap: () => context.go("/transactions/$_accountID"),
        child: Card(

      elevation: 2.0,
      shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(2.0))),
      child: Column(
        children: [
          Row(
            children: [SvgPicture.network(_bankImg, width: 16, height: 16,
            placeholderBuilder: (BuildContext ctx) => const CircularProgressIndicator()
            ,)], //ImageIcon(NetworkImage(_bankImg))
          ),
          Text(accountHolder),
          Text(accountNo),
          Row(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Column(
                children: [Text(avaliableBalance), Text(balance)],
              )
            ],
          )
        ],
      ),
    )));
  }
}

class AccountListBuilder extends StatelessWidget {
  const AccountListBuilder({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
        future: HomePageState.getAccounts(),
        builder: (ctx, snap) {
          if (snap.connectionState == ConnectionState.waiting ||
              (snap.connectionState == ConnectionState.done && !snap.hasData)) {
            return const Center(
                child: CircularProgressIndicator(
              color: Color(0x00BD1904),
              value: 50,
            ));
          } else if (snap.connectionState == ConnectionState.none) {
            if (!snap.hasError) {
              return const Center(child: Text("Something went wrong"));
            } else {
              var err = snap.error.toString();
              return Center(child: Text("This went wrong, $err"));
            }
          }
          if (snap.connectionState == ConnectionState.done) {
            return ListView(
              children: snap.data!,
            );
          }
          throw Exception("Unexpected end of function");
        });
  }
}

class DashboardContext {
  static GlobalKey<NavigatorState> navKey = GlobalKey<NavigatorState>();
}
