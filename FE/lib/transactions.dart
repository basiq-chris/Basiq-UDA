import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:localstorage/localstorage.dart';

class TransactionScreen extends StatelessWidget {

  Future<List<TableRow>> getTransactions() async {
    if (Uri.base.fragment == "/dashboard") {await Future.delayed(const Duration(milliseconds: 20));}
    String accID = Uri.base.fragment.split("/").last;
    List<TableRow> transactions = <TableRow>[];
    LocalStorage localStore = LocalStorage("currentSession");
    await localStore.ready;
    String payload = "${localStore.getItem("currentUser")}:$accID";
    payload = base64Encode(payload.codeUnits).toString();

    var trans = jsonDecode((await http.get(Uri.parse("http://localhost:8642/gettransactions/$payload"))).body);
    for (var t in trans["response_data"]["payload"]["transaction"]) {
      transactions.add(
        TableRow(
          children: [
            Text(t["postDate"].toString()),
            Text(t["description"].toString()),
            Text(t["amount"].toString())
          ]
        )
      );
    }

    return transactions;
  }


  @override
  Widget build(BuildContext context) {
    return FutureBuilder(future: getTransactions(), builder: (ctx, sn) {
      if (sn.connectionState == ConnectionState.waiting) {
        return const Scaffold(
          body: Column(
            children: [
              Row(
                  children:
                  [
                    Text("Fetching transactions")
                  ]
              ),
              CircularProgressIndicator()
            ],
          ),
        );
      }
      else if (sn.hasError) {
        return Scaffold(
          backgroundColor: Colors.red,
          body: Center(
            child: Text("ERROR FETCHING TRANSACTIONS\n ${sn.error!}\n\nIf this is not a Basiq error, contact the maintainer of this package"),
          ),
        );
      }
      else if (sn.connectionState == ConnectionState.done) {
        var transacData = sn.data!;
        return Scaffold(
          body: Column(
            children: [
              const Row(children: [Text("Account:")]),
              const Spacer(),
              Table(
                children: transacData,
              )
            ],
          ),
        );
      }
      throw Exception("Unknown Error");
    });
  }


}
//Needed data:
// date, amount, description