import 'dart:convert';

import 'package:fe/utils.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:localstorage/localstorage.dart';

class TransactionScreen extends StatelessWidget {
  const TransactionScreen({super.key});

  Future<List<TransacRow>> getTransactions() async {
    if (Uri.base.fragment == "/dashboard") {
      await Future.delayed(const Duration(milliseconds: 20));
    }
    String accID = Uri.base.fragment.split("/").last;
    List<TransacRow> transactions = <TransacRow>[];
    LocalStorage localStore = LocalStorage("currentSession");
    await localStore.ready;
    String payload = "${localStore.getItem("currentUser")}:$accID";
    payload = base64Encode(payload.codeUnits).toString();
    var trans = jsonDecode((await http
            .get(Uri.parse("http://localhost:8642/gettransactions/$payload")))
        .body);
    for (var t in trans["response_data"]["payload"]["transaction"]) {
      transactions.add(TransacRow(
          key: super.key,
          date: Utilities.parseRFC3339(t["postDate"].toString()),
          desc: t["description"].toString(),
          amt: t["amount"].toString()));
    }

    return transactions;
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
        future: getTransactions(),
        builder: (ctx, sn) {
          if (sn.connectionState == ConnectionState.waiting) {
            return const Scaffold(
              body: Column(
                children: [
                  Row(children: [Text("Fetching transactions")]),
                  CircularProgressIndicator()
                ],
              ),
            );
          } else if (sn.hasError) {
            return Scaffold(
              backgroundColor: Colors.red,
              body: Center(
                child: Text(
                    "ERROR FETCHING TRANSACTIONS\n ${sn.error!}\n\nIf this is not a Basiq error, contact the maintainer of this package"),
              ),
            );
          } else if (sn.connectionState == ConnectionState.done) {
            var transacData = sn.data!;
            return Scaffold(
              body: Expanded(
                child: Column(
                children: [
                  Row(children: [
                    Text("Account: ${Uri.base.fragment.split("/").last}")
                  ]),
                  Expanded(
                      child: ListView.builder(
                          itemCount: transacData.length,
                          prototypeItem: transacData[0],
                          itemBuilder: (ctx, idx) {
                            return transacData[idx];
                          }))
                ],
              ),
            ));
          }
          throw Exception("Unknown Error");
        });
  }
}

class TransacRow extends StatelessWidget {
  final String date, desc, amt;
  const TransacRow(
      {required super.key,
      required this.date,
      required this.desc,
      required this.amt});

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Row(
      children: [
        Text(date),
        Text(desc, textAlign: TextAlign.center,),
        Text(amt, textAlign: TextAlign.end,)
      ],
    ));
  }
}
