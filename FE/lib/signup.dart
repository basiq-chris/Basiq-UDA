import 'package:flutter/material.dart';

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
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: "Sign up",
      home: Scaffold(
        body: Center(
          child: Form(
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
                      validator: validateMobile,
                      obscureText: false,
                      decoration: const InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "Mobile"
                      ),
                    ),
                  ),
                ),
                const Padding(
                  padding: EdgeInsets.all(8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextField(
                      obscureText: false,
                      decoration: InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "First Name"
                      ),
                    ),
                  ),
                ),
                const Padding(
                  padding: EdgeInsets.all(8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextField(
                      obscureText: false,
                      decoration: InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "Middle Name"
                      ),
                    ),
                  ),
                ),
                const Padding(
                  padding: EdgeInsets.all(8.0),
                  child: SizedBox(
                    width: 350,
                    child: TextField(
                      obscureText: false,
                      decoration: InputDecoration(
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
                      ElevatedButton(onPressed: null, style: ElevatedButton.styleFrom(
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
  const pattern = r"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'"
      r'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-'
      r'\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*'
      r'[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4]'
      r'[0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9]'
      r'[0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\'
      r'x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])';
  final regex = RegExp(pattern);

  return value!.isNotEmpty && !regex.hasMatch(value)
      ? 'Enter a valid email address'
      : null;
}

String? validateMobile(String? value) {
  const pattern = r"^(\+\d{1,3}[ ]?)?\d{10}$";
  final regex = RegExp(pattern);

  return value!.isNotEmpty && !regex.hasMatch(value)
      ? "Invalid Mobile"
      : null;
}
