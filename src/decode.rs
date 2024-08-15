pub struct TextToken(u8);

impl TextToken {
  fn as_str(&self) -> &'static str {
    match self.0 {
      0x0 => "0",
      0x1 => "1",
      0x2 => "2",
      0x3 => "3",
      0x4 => "4",
      0x5 => "5",
      0x6 => "6",
      0x7 => "7",
      0x8 => "8",
      0x9 => "9",
      // no 0xa
      // no 0xb
      0xC => "#",
      0xD => ":",
      0xE => "/",
      0xF => "?",
      0x10 => ".",
      0x11 => "A",
      0x12 => "B",
      0x13 => "C",
      0x14 => "D",
      0x15 => "E",
      0x16 => "F",
      0x17 => "G",
      0x18 => "H",
      0x19 => "I",
      0x1A => "J",
      0x1B => "K",
      0x1C => "L",
      0x1D => "M",
      0x1E => "N",
      0x1F => "O",
      0x20 => "P",
      0x21 => "Q",
      0x22 => "R",
      0x23 => "S",
      0x24 => "T",
      0x25 => "U",
      0x26 => "V",
      0x27 => "W",
      0x28 => "X",
      0x29 => "Y",
      0x2A => "Z",
      0x2B => " ",
      0x2C => "'",
      0x2D => "-",
      0x2E => "_",
      0x2F => "*",
      0x30 => "AC POWER",
      0x31 => "ACCESS",
      0x32 => "ACCOUNT",
      0x33 => "ALARM",
      0x34 => "ALL",
      0x35 => "ARM",
      0x36 => "ARMING",
      0x37 => "AREA",
      0x38 => "ATTIC",
      0x39 => "AUTO",
      0x3A => "AUXILIARY",
      0x3B => "AWAY",
      0x3C => "BACK",
      0x3D => "BATTERY",
      0x3E => "BEDROOM",
      0x3F => "BEEPS",
      0x40 => "BOTTOM",
      0x41 => "BREEZEWAY",
      0x42 => "BASEMENT",
      0x43 => "BATHROOM",
      0x44 => "BUS",
      0x45 => "BYPASS",
      0x46 => "BYPASSED",
      0x47 => "CABINET",
      0x48 => "CANCELED",
      0x49 => "CARPET",
      0x4A => "CHIME",
      0x4B => "CLOSET",
      0x4C => "CLOSING",
      0x4D => "CODE",
      0x4E => "CONTROL",
      0x4F => "CPU",
      0x50 => "DEGREES",
      0x51 => "DEN",
      0x52 => "DESK",
      0x53 => "DELAY",
      0x54 => "DELETE",
      0x55 => "DINING",
      0x56 => "DIRECT",
      0x57 => "DOOR",
      0x58 => "DOWN",
      0x59 => "DOWNLOAD",
      0x5A => "DOWNSTAIRS",
      0x5B => "DRAWER",
      0x5C => "DISPLAY",
      0x5D => "DURESS",
      0x5E => "EAST",
      0x5F => "ENERGY SAVER",
      0x60 => "ENTER",
      0x61 => "ENTRY",
      0x62 => "ERROR",
      0x63 => "EXIT",
      0x64 => "FAIL",
      0x65 => "FAILURE",
      0x66 => "FAMILY",
      0x67 => "FEATURES",
      0x68 => "FIRE",
      0x69 => "FIRST",
      0x6A => "FLOOR",
      0x6B => "FORCE",
      0x6C => "FORMAT",
      0x6D => "FREEZE",
      0x6E => "FRONT",
      0x6F => "FURNACE",
      0x70 => "GARAGE",
      0x71 => "GALLERY",
      0x72 => "GOODBYE",
      0x73 => "GROUP",
      0x74 => "HALL",
      0x75 => "HEAT",
      0x76 => "HELLO",
      0x77 => "HELP",
      0x78 => "HIGH",
      0x79 => "HOURLY",
      0x7A => "HOUSE",
      0x7B => "IMMEDIATE",
      0x7C => "IN SERVICE",
      0x7D => "INTERIOR",
      0x7E => "INTRUSION",
      0x7F => "INVALID",
      0x80 => "IS",
      0x81 => "KEY",
      0x82 => "KITCHEN",
      0x83 => "LAUNDRY",
      0x84 => "LEARN",
      0x85 => "LEFT",
      0x86 => "LIBRARY",
      0x87 => "LEVEL",
      0x88 => "LIGHT",
      0x89 => "LIGHTS",
      0x8A => "LIVING",
      0x8B => "LOW",
      0x8C => "MAIN",
      0x8D => "MASTER",
      0x8E => "MEDICAL",
      0x8F => "MEMORY",
      0x90 => "MIN",
      0x91 => "MODE",
      0x92 => "MOTION",
      0x93 => "NIGHT",
      0x94 => "NORTH",
      0x95 => "NOT",
      0x96 => "NUMBER",
      0x97 => "OFF",
      0x98 => "OFFICE",
      0x99 => "OK",
      0x9A => "ON",
      0x9B => "OPEN",
      0x9C => "OPENING",
      0x9D => "PANIC",
      0x9E => "PARTITION",
      0x9F => "PATIO",
      0xA0 => "PHONE",
      0xA1 => "POLICE",
      0xA2 => "POOL",
      0xA3 => "PORCH",
      0xA4 => "PRESS",
      0xA5 => "QUIET",
      0xA6 => "QUICK",
      0xA7 => "RECEIVER",
      0xA8 => "REAR",
      0xA9 => "REPORT",
      0xAA => "REMOTE",
      0xAB => "RESTORE",
      0xAC => "RIGHT",
      0xAD => "ROOM",
      0xAE => "SCHEDULE",
      0xAF => "SCRIPT",
      0xB0 => "SEC",
      0xB1 => "SECOND",
      0xB2 => "SET",
      0xB3 => "SENSOR",
      0xB4 => "SHOCK",
      0xB5 => "SIDE",
      0xB6 => "SIREN",
      0xB7 => "SLIDING",
      0xB8 => "SMOKE",
      0xB9 => "Sn",
      0xBA => "SOUND",
      0xBB => "SOUTH",
      0xBC => "SPECIAL",
      0xBD => "STAIRS",
      0xBE => "START",
      0xBF => "STATUS",
      0xC0 => "STAY",
      0xC1 => "STOP",
      0xC2 => "SUPERVISORY",
      0xC3 => "SYSTEM",
      0xC4 => "TAMPER",
      0xC5 => "TEMPERATURE",
      0xC6 => "TEMPORARY",
      0xC7 => "TEST",
      0xC8 => "TIME",
      0xC9 => "TIMEOUT",
      0xCA => "TOUCHPAD",
      0xCB => "TRIP",
      0xCC => "TROUBLE",
      0xCD => "UNBYPASS",
      0xCE => "UNIT",
      0xCF => "UP",
      0xD0 => "VERIFY",
      0xD1 => "VIOLATION",
      0xD2 => "WARNING",
      0xD3 => "WEST",
      0xD4 => "WINDOW",
      0xD5 => "MENU",
      0xD6 => "RETURN",
      0xD7 => "POUND",
      0xD8 => "HOME",
      0xf9 => "\n",
      0xfa => "<spc>", // pseudo space
      0xfb => "\n",
      // no 0xfc
      0xfd => "<bs>",    // backspace
      0xfe => "<blink>", // blink next token
      _ => "@",
    }
  }
}

// https://github.com/JasonCarter80/concord232/blob/master/concord232/concord_tokens.py#L228
pub fn decode_text_tokens(tokens: &[u8]) -> String {
  let mut s = String::new();
  let n = tokens.len();

  for (i, t) in tokens.iter().enumerate() {
    if *t == 0xFD {
      s = s[..s.len() - 1].to_string();
      continue;
    }

    let c = TextToken(*t).as_str();
    s += c;

    if c.len() > 1 && i < n - 1 && !c.starts_with('<') {
      s += " ";
    }
  }

  s
}

pub fn letter_from_representative_hex(hex: u8) -> char {
  match hex {
    0x01 => 'A',
    0x02 => 'B',
    0x03 => 'C',
    0x04 => 'D',
    0x05 => 'E',
    0x06 => 'F',
    0x07 => 'G',
    0x08 => 'H',
    0x09 => 'I',
    0x10 => 'J',
    0x11 => 'K',
    0x12 => 'L',
    0x13 => 'M',
    0x14 => 'N',
    0x15 => 'O',
    0x16 => 'P',
    0x17 => 'Q',
    0x18 => 'R',
    0x19 => 'S',
    0x20 => 'T',
    0x21 => 'U',
    0x22 => 'V',
    0x23 => 'W',
    0x24 => 'X',
    0x25 => 'Y',
    0x26 => 'Z',
    _ => ' ',
  }
}
