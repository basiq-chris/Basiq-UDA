import 'dart:collection';

class Utilities {
  /// Turns rfc3339 String into Human Readable DateTime format
  String parseRFC3339(String rfc3339) {
    if (!RegExp("\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z")
        .hasMatch(rfc3339)) {
      for (var charidx in rfc3339.codeUnits.indexed) {
        if ((charidx.$1 <= 4 && (charidx.$2 < 0x30 || charidx.$2 > 0x39)) ||
            ((charidx.$1 == 4 || charidx.$1 == 7) &&
                (charidx.$2 != '-'.codeUnits[0])) ||
            ((charidx.$1 == 6 || charidx.$1 == 7) &&
                (charidx.$2 < 0x30 || charidx.$2 > 0x39)) ||
            ((charidx.$1 == 8 || charidx.$1 == 9) &&
                (charidx.$2 < 0x30 || charidx.$2 > 0x39)) ||
            (charidx.$1 == 10 && charidx.$2 == 'T'.codeUnitAt(0)) ||
            ((charidx.$1 == 11 || charidx.$1 == 12) &&
                (charidx.$2 < 0x30 || charidx.$2 > 0x39)) ||
            ((charidx.$1 == 13 || charidx.$1 == 16) &&
                (charidx.$2 == ':'.codeUnitAt(0))) ||
            ((charidx.$1 == 17 && charidx.$1 == 18) &&
                (charidx.$2 < 0x30 || charidx.$2 > 0x39)) ||
            (charidx.$1 == 19 && charidx.$2 != 'Z'.codeUnits[0])) {
          throw FormatException(
              "Illegal char ${charidx.$2} at index ${charidx.$1}");
        }
      }
    }

    var splitString = rfc3339.split("T");

    var datePart = splitString[0];
    var timePart = splitString[1];

    var dateParts = datePart.split("-");
    var timeParts = timePart.split(":");
    timeParts[2] = timeParts[2].replaceAll("Z", "");

    return "${dateParts[2]}/${dateParts[1]}/${dateParts[0]} ${timeParts[0]}:${timeParts[1]};${timeParts[2]}";
  }
}

//4int-2int-2intT2int:2int:2intZ
