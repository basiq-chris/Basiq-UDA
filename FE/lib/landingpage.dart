
import 'dart:js';

import 'package:fe/callback.dart';
import 'package:fe/homepage.dart';
import 'package:fe/signup.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

void main() => runApp(const Application());

class Application extends StatelessWidget {
  const Application({super.key});
  @override
  Widget build(BuildContext context) {
    return const RouterWidget();
  }
}

final _router = GoRouter(routes: [
  GoRoute(path: '/',
      builder: (context, state) => const LandingPageState()),
  GoRoute(path: '/home',
      builder: (context, state) => const LandingPageState()),
  GoRoute(path: '/signup',
      builder: (context, state) => const SignupPage()),
  GoRoute(path: "/dashboard",
      builder: (context, state) {Job job = state.extra as Job;
                                    return HomePage(jobID: job);}),
  GoRoute(path: "/callback",
      builder: (context, state) => Callback())
]);

class RouterWidget extends StatelessWidget {
  const RouterWidget({super.key});

  @override
  Widget build(BuildContext context) {
      return MaterialApp.router(
        routerConfig: _router,
        title: "SallyAx",
      );
  }
}

class LandingPageState extends StatefulWidget {
  const LandingPageState({super.key});

  @override
  State<StatefulWidget> createState() => LandingPage();

}

class LandingPage extends State<LandingPageState> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Form(
          key: _formKey,
          child: Scaffold(
            body: Padding(
              padding: const EdgeInsets.all(18.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Padding(
                    padding: const EdgeInsets.all(8.0),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        const Text("SallyAX App",
                          style: TextStyle(
                              fontSize: 36
                          ),),
                        Padding(
                          padding: const EdgeInsets.all(16.0),
                          child: Image.asset('Assets/images/SallyAx.png'),
                        )
                      ],
                    ),
                  ),
                  const Padding(
                    padding: EdgeInsets.only(bottom: 12.0),
                    child: SizedBox(
                      width: 250,
                      child: TextField(
                        obscureText: false,
                        decoration: InputDecoration(
                            border: OutlineInputBorder(),
                            labelText: "Username"
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(
                    width: 250,
                    child: TextField(
                      obscureText: true,
                      decoration: InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: "Password"
                      ),
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.all(15.0),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Padding(
                          padding: const EdgeInsets.only(right: 18.0),
                          child: ElevatedButton(onPressed: () => {context.go('/signup')},
                              style: ElevatedButton.styleFrom(
                                backgroundColor: const Color(0xFF00A5FF),
                                foregroundColor: const Color(0xFF000000),
                              ), child: const Text("Sign up")),
                        ),
                        ElevatedButton(onPressed: null,
                          style: ElevatedButton.styleFrom(
                              foregroundColor: const Color(0xFF000000),
                              backgroundColor: Colors.lightGreen
                          ), child: const Text("Sign in"),)
                      ],
                    ),
                  )
                ],
              ),
            ),
          ),
        )
    );
  }

}