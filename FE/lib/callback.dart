
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class Callback extends StatelessWidget {
  const Callback({super.key});

  @override
  Widget build(BuildContext context) {
    String jobID = Uri.parse(Uri.base.toString().replaceAll("#/", "")).queryParameters["jobId"].toString();
    context.go('/dashboard', extra: Job(jobID: jobID));

    return const MaterialApp(
      title: "Redirecting to dashboard",
    );
  }

}

class Job {
  String jobID;
  Job({required this.jobID});
}