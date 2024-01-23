import 'dart:collection';

class Utilities {
  /// Turns rfc3339 String into Human Readable DateTime format
  // "2005-08-15T15:52:01+00:00"
  static String parseRFC3339(String rfc3339) {
    if (!RegExp("\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z\$")
        .hasMatch(rfc3339)) {
      bool isDigit(int char) {
        return char >= 0x30 && char <= 0x39;
      }
      bool isColon(int char) {
        return char == ':'.codeUnits[0];
      }
      bool isDash(int char) {
        return char == '-'.codeUnits[0];
      }
      bool isDateDelimiter(int char) {
        return char == 'T'.codeUnits[0];
      }
      bool isUTCDelimiter(int char) {
        return char == 'Z'.codeUnits[0];
      }
      void throws(int idx, int char) {
        throw FormatException("Illegal char ${String.fromCharCode(char)} at index $idx}");
      }
      for (var charidx in rfc3339.codeUnits.indexed) {
        switch (charidx.$1) {
          case 0:
          case 1:
          case 2:
          case 3:
          case 5:
          case 6:
          case 8:
          case 9:
          case 11:
          case 12:
          case 14:
          case 15:
          case 17:
          case 18:
            if (!isDigit(charidx.$2)) {throws(charidx.$1, charidx.$2);}
            continue;
          case 4:
          case 7:
            if (!isDash(charidx.$2)) {throws(charidx.$1, charidx.$2);}
            continue;
          case 10:
            if (!isDateDelimiter(charidx.$2)) {throws(charidx.$1, charidx.$2);};
            continue;
          case 13:
          case 16:
            if (!isColon(charidx.$2)) {throws(charidx.$1, charidx.$2);}
            continue;
          case 19:
            if (!isUTCDelimiter(charidx.$2)) {throws(charidx.$1, charidx.$2);}
            break;
          default:
            throw const FormatException("String too large");
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
