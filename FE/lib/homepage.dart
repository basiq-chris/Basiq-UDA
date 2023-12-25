

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
    LocalStorage localStore = LocalStorage("currentSession");
    List<Widget> accounts = [];

    return MaterialApp(
      title: "Dashboard",
      home: Scaffold(
          body: ListView.builder(
            itemCount: accounts.length,
          ),
      ),
    );
  }
}

class AccountChip extends StatefulWidget {


  @override
  State<StatefulWidget> createState() => _AccountChipState();

}

class _AccountChipState extends State<AccountChip> {
  late final String accountName, balance, avaliableFunds, accountNo;

  void setInfo(Map<String, dynamic> accJSON) {
    setState(() {
      accountName = accJSON["accountHolder"].toString();
      balance = accJSON["balance"].toString();
      avaliableFunds = accJSON["avaliableFunds"].toString();
      accountNo = accJSON["accountNo"].toString();
    });
  }

  @override
  Widget build(BuildContext context) {
    // TODO: implement build
    throw UnimplementedError();
  }

}