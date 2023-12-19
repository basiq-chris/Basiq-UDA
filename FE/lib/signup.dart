import 'dart:convert';
import 'dart:core';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:localstorage/localstorage.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:url_launcher/url_launcher_string.dart';

class SignupPage extends StatelessWidget {
  const SignupPage({super.key});

  @override
  Widget build(BuildContext context) {
    return const SignupState();
  }

}

class SignupState extends StatefulWidget {
  const SignupState({super.key});

  @override
  State<StatefulWidget> createState() => SignupForm();

}

class SignupForm extends State<SignupState> {
  final _formKey = GlobalKey<FormState>();
  late LocalStorage lclStg;
  late dynamic userResponseJson, authLinkResponseJson;
  late http.Response userResponse, authLinkResponse;
  var state = {
    "email": "",
    "mobile": "",
    "firstName": "",
    "middleName": "",
    "lastName": ""
  };
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: "Sign up",
      home: Scaffold(
        body: Center(
          child: Form(
            key: _formKey,
            autovalidateMode: AutovalidateMode.always,
            child: Column(
              children: [
                const Padding(
                  padding: EdgeInsets.all(8.0),
                  child: Text("Let the Journey Begin", style: TextStyle(fontSize: 34, fontWeight: FontWeight.w500),),
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 32.0),
                  child: SizedBox(
                    width: 350,
                    child: TextFormField(
                      onSaved: (String? val) {state["email"] = val ??= "";},
                      obscureText: false,
                      validator: validateEmail,
                      decoration: const InputDecoration(
                        border: OutlineInputBorder(),
                        labelText: "Email"
                      ),
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 16.0, bottom: 8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextFormField(
                      onSaved: (String? val) {state["mobile"] = val ??= "";},
                      validator: validateMobile,
                      obscureText: false,
                      decoration: const InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "Mobile"
                      ),
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextFormField(
                      onSaved: (String? val) {state["firstName"] = val ??= "";},
                      obscureText: false,
                      decoration: const InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "First Name"
                      ),
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextFormField(
                      onSaved: (String? val) {state["middleName"] = val ??= "";},
                      obscureText: false,
                      decoration: const InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "Middle Name"
                      ),
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextFormField(
                        onSaved: (String? val) {state["lastName"] = val ??= "";},
                      obscureText: false,
                      decoration: const InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "Surname"
                      ),
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      ElevatedButton(onPressed: () async => {
                        if (_formKey.currentState!.validate()) {
                          _formKey.currentState!.save(),
                          userResponse = (await http.post(Uri.parse("http://127.0.0.1:8642/createuser"), body: state)),
                          //TODO: Send entire response to logger when implemented
                          //debugPrint(String.fromCharCodes(user_response.bodyBytes)),
                          userResponseJson = json.decode(userResponse.body),
                          lclStg = LocalStorage(userResponseJson["response_data"]["payload"]["id"].toString()),
                          lclStg.setItem("currentUser", userResponseJson["response_data"]["payload"]["id"].toString()),
                          lclStg.setItem("userPayload", userResponseJson["response_data"]["payload"].toString()),
                          //debugPrint(user_response_json["response_data"]["payload"]["id"]),
                          authLinkResponse = await http.post(Uri.parse("http://127.0.0.1:8642/createauthlink"), body: {"userID": userResponseJson["response_data"]["payload"]["id"]}),
                          authLinkResponseJson = jsonDecode(authLinkResponse.body),
                          if (await canLaunchUrlString(authLinkResponseJson["response_data"]["payload"]["authLink"].toString())) {
                            launchUrl(Uri.parse(authLinkResponseJson["response_data"]["payload"]["authLink"].toString()), webOnlyWindowName: "_self")
                          } else {
                            throw Exception("url cannot be linked to")
                          }
                        }
                      }, style: ElevatedButton.styleFrom(
                        foregroundColor: const Color(0xFF000000),
                        backgroundColor: const Color(0xFFA5FFFF),
                        minimumSize: const Size(355, 50)
                      ),
                      child: const Text("Sign up"),)
                    ],
                  ),
                )
              ],
            ),
          ),
        )
      ),
    );
  }
}

String? validateEmail(String? value) {
  if (value == null || value == "") {
    return "Email must be provided";
  }
  const pattern = r"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'"
      r'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-'
      r'\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*'
      r'[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4]'
      r'[0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9]'
      r'[0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\'
      r'x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])';
  final regex = RegExp(pattern);

  return value.isNotEmpty && !regex.hasMatch(value)
      ? 'Enter a valid email address'
      : null;
}

String? validateMobile(String? value) {
  if (value == null || value == "") {
    return "Mobile number must be provided";
  }
  const pattern = r"\++\d{2,3}\d{9,10}";
  final regex = RegExp(pattern);

  return value.isNotEmpty && !regex.hasMatch(value)
      ? "Invalid Mobile"
      : null;
}

