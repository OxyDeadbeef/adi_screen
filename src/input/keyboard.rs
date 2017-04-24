/**
 * adi_screen - Aldaron's Device Interface - Screen - "input/keyboard.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use input::Key;

// Non-Printing Characters
const ESCAPE : Key = Key::Escape;
const BACKSPACE : Key = Key::Backspace;
const F1 : Key = Key::F(1);
const F2 : Key = Key::F(2);
const F3 : Key = Key::F(3);
const F4 : Key = Key::F(4);
const F5 : Key = Key::F(5);
const F6 : Key = Key::F(6);
const F7 : Key = Key::F(7);
const F8 : Key = Key::F(8);
const F9 : Key = Key::F(9);
const F10 : Key = Key::F(10);
const F11 : Key = Key::F(11);
const F12 : Key = Key::F(12);
const LCTRL : Key = Key::Ctrl(false);
const RCTRL : Key = Key::Ctrl(true);
const LALT : Key = Key::Alt(false);
const RALT : Key = Key::Alt(true);
const LSHIFT : Key = Key::Shift(false);
const RSHIFT : Key = Key::Shift(true);
const CAPS_LOCK : Key = Key::CapsLock;
const NUM_LOCK : Key = Key::NumLock;
const HOME : Key = Key::Home;
const END : Key = Key::End;
const PAGE_UP : Key = Key::PageUp;
const PAGE_DOWN : Key = Key::PageDown;
const UP : Key = Key::Up;
const DOWN : Key = Key::Down;
const RIGHT : Key = Key::Right;
const LEFT : Key = Key::Left;
const INSERT : Key = Key::Insert;
const DELETE : Key = Key::Delete;

// Printing Characters
const BACKTICK : Key = Key::Char('`');
const NUM1 : Key = Key::Char('1');
const NUM2 : Key = Key::Char('2');
const NUM3 : Key = Key::Char('3');
const NUM4 : Key = Key::Char('4');
const NUM5 : Key = Key::Char('5');
const NUM6 : Key = Key::Char('6');
const NUM7 : Key = Key::Char('7');
const NUM8 : Key = Key::Char('8');
const NUM9 : Key = Key::Char('9');
const NUM0 : Key = Key::Char('0');
const MINUS : Key = Key::Char('-');
const PLUS : Key = Key::Char('+');
const EQUALS : Key = Key::Char('=');
const TAB : Key = Key::Char('\t');
const KEYA : Key = Key::Char('a');
const KEYB : Key = Key::Char('b');
const KEYC : Key = Key::Char('c');
const KEYD : Key = Key::Char('d');
const KEYE : Key = Key::Char('e');
const KEYF : Key = Key::Char('f');
const KEYG : Key = Key::Char('g');
const KEYH : Key = Key::Char('h');
const KEYI : Key = Key::Char('i');
const KEYJ : Key = Key::Char('j');
const KEYK : Key = Key::Char('k');
const KEYL : Key = Key::Char('l');
const KEYM : Key = Key::Char('m');
const KEYN : Key = Key::Char('n');
const KEYO : Key = Key::Char('o');
const KEYP : Key = Key::Char('p');
const KEYQ : Key = Key::Char('q');
const KEYR : Key = Key::Char('r');
const KEYS : Key = Key::Char('s');
const KEYT : Key = Key::Char('t');
const KEYU : Key = Key::Char('u');
const KEYV : Key = Key::Char('v');
const KEYW : Key = Key::Char('w');
const KEYX : Key = Key::Char('x');
const KEYY : Key = Key::Char('y');
const KEYZ : Key = Key::Char('z');
const BRACKETOPEN : Key = Key::Char('[');
const BRACKETCLOSE : Key = Key::Char(']');
const NEWLINE : Key = Key::Char('\n');
const BACKSLASH : Key = Key::Char('\\');
const FORWARDSLASH : Key = Key::Char('/');
const SEMICOLON : Key = Key::Char(';');
const QUOTE : Key = Key::Char('\'');
const COMMA : Key = Key::Char(',');
const PERIOD : Key = Key::Char('.');
const ASTERISK : Key = Key::Char('*');
const SPACE : Key = Key::Char(' ');

#[cfg(target_os = "windows")]
pub fn english(physical_key: u16, scan_key: u16) -> Key {
	match physical_key {
		18 => match scan_key {
			0b100111000 => RALT,
			_ => LALT,
		},
		27 => ESCAPE,
		192 => BACKTICK,
		48 | 96 => NUM0,
		49 | 97 => NUM1,
		50 | 98 => NUM2,
		51 | 99 => NUM3,
		52 | 100 => NUM4,
		53 | 101 => NUM5,
		54 | 102 => NUM6,
		55 | 103 => NUM7,
		56 | 104 => NUM8,
		57 | 105 => NUM9,
		189 | 109 => MINUS,
		187 => EQUALS,
		8 => BACKSPACE,
		9 => TAB,
		65 => KEYA,
		66 => KEYB,
		67 => KEYC,
		68 => KEYD,
		69 => KEYE,
		70 => KEYF,
		71 => KEYG,
		72 => KEYH,
		73 => KEYI,
		74 => KEYJ,
		75 => KEYK,
		76 => KEYL,
		77 => KEYM,
		78 => KEYN,
		79 => KEYO,
		80 => KEYP,
		81 => KEYQ,
		82 => KEYR,
		83 => KEYS,
		84 => KEYT,
		85 => KEYU,
		86 => KEYV,
		87 => KEYW,
		88 => KEYX,
		89 => KEYY,
		90 => KEYZ,
		219 => BRACKETOPEN,
		221 => BRACKETCLOSE,
		13 => NEWLINE,
		17 => match scan_key {
			0b100011101 => RCTRL,
			_ => LCTRL,
		},
		186 => SEMICOLON,
		222 => QUOTE,
		16 => match scan_key {
			0b110110 => RSHIFT,
			_ => LSHIFT,
		},
		220 => BACKSLASH,
		188 => COMMA,
		190 | 110 => PERIOD,
		106 => ASTERISK,
		32 => SPACE,
		20 => CAPS_LOCK,
		112 => F1,
		113 => F2,
		114 => F3,
		115 => F4,
		116 => F5,
		117 => F6,
		118 => F7,
		119 => F8,
		120 => F9,
		121 => F10,
		122 => F11,
		123 => F12,
		144 => NUM_LOCK,
		107 => PLUS,
		111 | 191 => FORWARDSLASH,
		36 => HOME,
		33 => PAGE_UP,
		34 => PAGE_DOWN,
		35 => END,
		37 => LEFT,
		38 => UP,
		39 => RIGHT,
		40 => DOWN,
		45 => INSERT,
		46 => DELETE,
		_ => Key::Char('\x00'),
	}
}

#[cfg(target_os = "linux")]
pub fn english(physical_key: u32) -> Key {
	match physical_key {
		9 => ESCAPE,
		49 => BACKTICK,
		10 | 87 => NUM1,
		11 | 88 => NUM2,
		12 | 89 => NUM3,
		13 | 83 => NUM4,
		14 | 84 => NUM5,
		15 | 85 => NUM6,
		16 | 79 => NUM7,
		17 | 80 => NUM8,
		18 | 81 => NUM9,
		19 | 90 => NUM0,
		20 | 82 => MINUS,
		21 => EQUALS,
		22 => BACKSPACE,
		23 => TAB,
		24 => KEYQ,
		25 => KEYW,
		26 => KEYE,
		27 => KEYR,
		28 => KEYT,
		29 => KEYY,
		30 => KEYU,
		31 => KEYI,
		32 => KEYO,
		33 => KEYP,
		34 => BRACKETOPEN,
		35 => BRACKETCLOSE,
		36 | 104 => NEWLINE,
		37 => LCTRL,
		38 => KEYA,
		39 => KEYS,
		40 => KEYD,
		41 => KEYF,
		42 => KEYG,
		43 => KEYH,
		44 => KEYJ,
		45 => KEYK,
		46 => KEYL,
		47 => SEMICOLON,
		48 => QUOTE,
		50 => LSHIFT,
		51 => BACKSLASH,
		52 => KEYZ,
		53 => KEYX,
		54 => KEYC,
		55 => KEYV,
		56 => KEYB,
		57 => KEYN,
		58 => KEYM,
		59 => COMMA,
		60 | 91 => PERIOD,
		62 => RSHIFT,
		63 => ASTERISK,
		64 => LALT,
		65 => SPACE,
		66 => CAPS_LOCK,
		67 => F1,
		68 => F2,
		69 => F3,
		70 => F4,
		71 => F5,
		72 => F6,
		73 => F7,
		74 => F8,
		75 => F9,
		76 => F10,
		77 => NUM_LOCK,
		86 => PLUS,
		95 => F11,
		96 => F12,
		105 => RCTRL,
		106 | 61 => FORWARDSLASH,
		108 => RALT,
		110 => HOME,
		111 => UP,
		112 => PAGE_UP,
		113 => LEFT,
		114 => RIGHT,
		115 => END,
		116 => DOWN,
		117 => PAGE_DOWN,
		118 => INSERT,
		119 => DELETE,
		_ => Key::Char('\x00'),
	}
}
