import 'package:fe/utils.dart';
import 'package:test/test.dart';

void main() {
  test("Valid RFC3339", () {
    const validRFC3339 = "2023-01-07T03:04:05Z";

    expect(() => Utilities.parseRFC3339(validRFC3339), returnsNormally);
  });

  test("Invalid RFC3339", () {
    const invalidRFC3339 = "2023J01:07Z03:44:00T";

    expect(() => Utilities.parseRFC3339(invalidRFC3339), throwsFormatException);
  });

  test("Invalid UTC UFC3339", () {
    const shiftedRFC3339 = "2023-01-07T03:04:05Z+11:00";

    expect(() => Utilities.parseRFC3339(shiftedRFC3339), throwsFormatException);
  });
}