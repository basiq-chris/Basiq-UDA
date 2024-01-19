import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:localstorage/localstorage.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class TransactionScreen extends StatelessWidget {

  Future<List<TableRow>> getTransactions(String accID) async {
    List<TableRow> transactions = <TableRow>[];
    LocalStorage localStore = LocalStorage("currentSession");
    await localStore.ready;
    String payload = localStore.getItem("userID") + ":" + accID;
    payload = base64Encode(payload.codeUnits).toString();

    var trans = jsonDecode((await http.get(Uri.parse("http://localhost:8642/gettransactions/$payload"))).body);
    for (var t in trans["response_data"]["payload"]["transactions"]) {
      transactions.add(
        TableRow(
          children: [
            Text(t["date"].toString()),
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
    String accID = Uri.base.path.split("/").last;
    return FutureBuilder(future: getTransactions(accID), builder: (ctx, sn) {
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
        return const Scaffold(
          backgroundColor: Colors.red,
          body: Center(
            child: Text("ERROR FETCHING TRANSACTIONS"),
          ),
        );
      }
      else if (sn.connectionState == ConnectionState.done) {
        var transacData = sn.data!;
        return Scaffold(
          body: Column(
            children: [
              Row(children: [Text("Account: $accID")]),
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