/* Note: This file is an incomplete collection of type declarations to facilitate use of
 * TypeScript with QML. If a property or class you need is missing, please consult the
 * Qt Documentation and add it!
 * */

declare namespace Qt {
  const NoButton = 0x00000000;
  const AllButtons = 0x07ffffff;
  const LeftButton = 0x00000001;
  const RightButton = 0x00000002;
  const MidButton = 0x00000004;
  const MiddleButton = 0x00000004;
  const BackButton = 0x00000008;
  const XButton1 = 0x00000008;
  const ExtraButton1 = 0x00000008;
  const ForwardButton = 0x00000010;
  const XButton2 = 0x00000010;
  const ExtraButton2 = 0x00000010;
  const TaskButton = 0x00000020;
  const ExtraButton3 = 0x00000020;
  const ExtraButton4 = 0x00000040;
  const ExtraButton5 = 0x00000080;
  const ExtraButton6 = 0x00000100;
  const ExtraButton7 = 0x00000200;
  const ExtraButton8 = 0x00000400;
  const ExtraButton9 = 0x00000800;
  const ExtraButton10 = 0x00001000;
  const ExtraButton11 = 0x00002000;
  const ExtraButton12 = 0x00004000;
  const ExtraButton13 = 0x00008000;
  const ExtraButton14 = 0x00010000;
  const ExtraButton15 = 0x00020000;
  const ExtraButton16 = 0x00040000;
  const ExtraButton17 = 0x00080000;
  const ExtraButton18 = 0x00100000;
  const ExtraButton19 = 0x00200000;
  const ExtraButton20 = 0x00400000;
  const ExtraButton21 = 0x00800000;
  const ExtraButton22 = 0x01000000;
  const ExtraButton23 = 0x02000000;
  const ExtraButton24 = 0x04000000;

  const Key_Escape = 0x01000000;
  const Key_Tab = 0x01000001;
  const Key_Backtab = 0x01000002;
  const Key_Backspace = 0x01000003;
  const Key_Return = 0x01000004;
  const Key_Enter = 0x01000005;
  const Key_Insert = 0x01000006;
  const Key_Delete = 0x01000007;
  const Key_Pause = 0x01000008;
  const Key_Print = 0x01000009;
  const Key_SysReq = 0x0100000a;
  const Key_Clear = 0x0100000b;
  const Key_Home = 0x01000010;
  const Key_End = 0x01000011;
  const Key_Left = 0x01000012;
  const Key_Up = 0x01000013;
  const Key_Right = 0x01000014;
  const Key_Down = 0x01000015;
  const Key_PageUp = 0x01000016;
  const Key_PageDown = 0x01000017;
  const Key_Shift = 0x01000020;
  const Key_Control = 0x01000021;
  const Key_Meta = 0x01000022;
  const Key_Alt = 0x01000023;
  const Key_AltGr = 0x01001103;
  const Key_CapsLock = 0x01000024;
  const Key_NumLock = 0x01000025;
  const Key_ScrollLock = 0x01000026;
  const Key_F1 = 0x01000030;
  const Key_F2 = 0x01000031;
  const Key_F3 = 0x01000032;
  const Key_F4 = 0x01000033;
  const Key_F5 = 0x01000034;
  const Key_F6 = 0x01000035;
  const Key_F7 = 0x01000036;
  const Key_F8 = 0x01000037;
  const Key_F9 = 0x01000038;
  const Key_F10 = 0x01000039;
  const Key_F11 = 0x0100003a;
  const Key_F12 = 0x0100003b;
  const Key_F13 = 0x0100003c;
  const Key_F14 = 0x0100003d;
  const Key_F15 = 0x0100003e;
  const Key_F16 = 0x0100003f;
  const Key_F17 = 0x01000040;
  const Key_F18 = 0x01000041;
  const Key_F19 = 0x01000042;
  const Key_F20 = 0x01000043;
  const Key_F21 = 0x01000044;
  const Key_F22 = 0x01000045;
  const Key_F23 = 0x01000046;
  const Key_F24 = 0x01000047;
  const Key_F25 = 0x01000048;
  const Key_F26 = 0x01000049;
  const Key_F27 = 0x0100004a;
  const Key_F28 = 0x0100004b;
  const Key_F29 = 0x0100004c;
  const Key_F30 = 0x0100004d;
  const Key_F31 = 0x0100004e;
  const Key_F32 = 0x0100004f;
  const Key_F33 = 0x01000050;
  const Key_F34 = 0x01000051;
  const Key_F35 = 0x01000052;
  const Key_Super_L = 0x01000053;
  const Key_Super_R = 0x01000054;
  const Key_Menu = 0x01000055;
  const Key_Hyper_L = 0x01000056;
  const Key_Hyper_R = 0x01000057;
  const Key_Help = 0x01000058;
  const Key_Direction_L = 0x01000059;
  const Key_Direction_R = 0x01000060;
  const Key_Space = 0x20;
  const Key_Any = 0x20;
  const Key_Exclam = 0x21;
  const Key_QuoteDbl = 0x22;
  const Key_NumberSign = 0x23;
  const Key_Dollar = 0x24;
  const Key_Percent = 0x25;
  const Key_Ampersand = 0x26;
  const Key_Apostrophe = 0x27;
  const Key_ParenLeft = 0x28;
  const Key_ParenRight = 0x29;
  const Key_Asterisk = 0x2a;
  const Key_Plus = 0x2b;
  const Key_Comma = 0x2c;
  const Key_Minus = 0x2d;
  const Key_Period = 0x2e;
  const Key_Slash = 0x2f;
  const Key_0 = 0x30;
  const Key_1 = 0x31;
  const Key_2 = 0x32;
  const Key_3 = 0x33;
  const Key_4 = 0x34;
  const Key_5 = 0x35;
  const Key_6 = 0x36;
  const Key_7 = 0x37;
  const Key_8 = 0x38;
  const Key_9 = 0x39;
  const Key_Colon = 0x3a;
  const Key_Semicolon = 0x3b;
  const Key_Less = 0x3c;
  const Key_Equal = 0x3d;
  const Key_Greater = 0x3e;
  const Key_Question = 0x3f;
  const Key_At = 0x40;
  const Key_A = 0x41;
  const Key_B = 0x42;
  const Key_C = 0x43;
  const Key_D = 0x44;
  const Key_E = 0x45;
  const Key_F = 0x46;
  const Key_G = 0x47;
  const Key_H = 0x48;
  const Key_I = 0x49;
  const Key_J = 0x4a;
  const Key_K = 0x4b;
  const Key_L = 0x4c;
  const Key_M = 0x4d;
  const Key_N = 0x4e;
  const Key_O = 0x4f;
  const Key_P = 0x50;
  const Key_Q = 0x51;
  const Key_R = 0x52;
  const Key_S = 0x53;
  const Key_T = 0x54;
  const Key_U = 0x55;
  const Key_V = 0x56;
  const Key_W = 0x57;
  const Key_X = 0x58;
  const Key_Y = 0x59;
  const Key_Z = 0x5a;
  const Key_BracketLeft = 0x5b;
  const Key_Backslash = 0x5c;
  const Key_BracketRight = 0x5d;
  const Key_AsciiCircum = 0x5e;
  const Key_Underscore = 0x5f;
  const Key_QuoteLeft = 0x60;
  const Key_BraceLeft = 0x7b;
  const Key_Bar = 0x7c;
  const Key_BraceRight = 0x7d;
  const Key_AsciiTilde = 0x7e;
  const Key_nobreakspace = 0x0a0;
  const Key_exclamdown = 0x0a1;
  const Key_cent = 0x0a2;
  const Key_sterling = 0x0a3;
  const Key_currency = 0x0a4;
  const Key_yen = 0x0a5;
  const Key_brokenbar = 0x0a6;
  const Key_section = 0x0a7;
  const Key_diaeresis = 0x0a8;
  const Key_copyright = 0x0a9;
  const Key_ordfeminine = 0x0aa;
  const Key_guillemotleft = 0x0ab;
  const Key_notsign = 0x0ac;
  const Key_hyphen = 0x0ad;
  const Key_registered = 0x0ae;
  const Key_macron = 0x0af;
  const Key_degree = 0x0b0;
  const Key_plusminus = 0x0b1;
  const Key_twosuperior = 0x0b2;
  const Key_threesuperior = 0x0b3;
  const Key_acute = 0x0b4;
  const Key_mu = 0x0b5;
  const Key_paragraph = 0x0b6;
  const Key_periodcentered = 0x0b7;
  const Key_cedilla = 0x0b8;
  const Key_onesuperior = 0x0b9;
  const Key_masculine = 0x0ba;
  const Key_guillemotright = 0x0bb;
  const Key_onequarter = 0x0bc;
  const Key_onehalf = 0x0bd;
  const Key_threequarters = 0x0be;
  const Key_questiondown = 0x0bf;
  const Key_Agrave = 0x0c0;
  const Key_Aacute = 0x0c1;
  const Key_Acircumflex = 0x0c2;
  const Key_Atilde = 0x0c3;
  const Key_Adiaeresis = 0x0c4;
  const Key_Aring = 0x0c5;
  const Key_AE = 0x0c6;
  const Key_Ccedilla = 0x0c7;
  const Key_Egrave = 0x0c8;
  const Key_Eacute = 0x0c9;
  const Key_Ecircumflex = 0x0ca;
  const Key_Ediaeresis = 0x0cb;
  const Key_Igrave = 0x0cc;
  const Key_Iacute = 0x0cd;
  const Key_Icircumflex = 0x0ce;
  const Key_Idiaeresis = 0x0cf;
  const Key_ETH = 0x0d0;
  const Key_Ntilde = 0x0d1;
  const Key_Ograve = 0x0d2;
  const Key_Oacute = 0x0d3;
  const Key_Ocircumflex = 0x0d4;
  const Key_Otilde = 0x0d5;
  const Key_Odiaeresis = 0x0d6;
  const Key_multiply = 0x0d7;
  const Key_Ooblique = 0x0d8;
  const Key_Ugrave = 0x0d9;
  const Key_Uacute = 0x0da;
  const Key_Ucircumflex = 0x0db;
  const Key_Udiaeresis = 0x0dc;
  const Key_Yacute = 0x0dd;
  const Key_THORN = 0x0de;
  const Key_ssharp = 0x0df;
  const Key_division = 0x0f7;
  const Key_ydiaeresis = 0x0ff;
  const Key_Multi_key = 0x01001120;
  const Key_Codeinput = 0x01001137;
  const Key_SingleCandidate = 0x0100113c;
  const Key_MultipleCandidate = 0x0100113d;
  const Key_PreviousCandidate = 0x0100113e;
  const Key_Mode_switch = 0x0100117e;
  const Key_Kanji = 0x01001121;
  const Key_Muhenkan = 0x01001122;
  const Key_Henkan = 0x01001123;
  const Key_Romaji = 0x01001124;
  const Key_Hiragana = 0x01001125;
  const Key_Katakana = 0x01001126;
  const Key_Hiragana_Katakana = 0x01001127;
  const Key_Zenkaku = 0x01001128;
  const Key_Hankaku = 0x01001129;
  const Key_Zenkaku_Hankaku = 0x0100112a;
  const Key_Touroku = 0x0100112b;
  const Key_Massyo = 0x0100112c;
  const Key_Kana_Lock = 0x0100112d;
  const Key_Kana_Shift = 0x0100112e;
  const Key_Eisu_Shift = 0x0100112f;
  const Key_Eisu_toggle = 0x01001130;
  const Key_Hangul = 0x01001131;
  const Key_Hangul_Start = 0x01001132;
  const Key_Hangul_End = 0x01001133;
  const Key_Hangul_Hanja = 0x01001134;
  const Key_Hangul_Jamo = 0x01001135;
  const Key_Hangul_Romaja = 0x01001136;
  const Key_Hangul_Jeonja = 0x01001138;
  const Key_Hangul_Banja = 0x01001139;
  const Key_Hangul_PreHanja = 0x0100113a;
  const Key_Hangul_PostHanja = 0x0100113b;
  const Key_Hangul_Special = 0x0100113f;
  const Key_Dead_Grave = 0x01001250;
  const Key_Dead_Acute = 0x01001251;
  const Key_Dead_Circumflex = 0x01001252;
  const Key_Dead_Tilde = 0x01001253;
  const Key_Dead_Macron = 0x01001254;
  const Key_Dead_Breve = 0x01001255;
  const Key_Dead_Abovedot = 0x01001256;
  const Key_Dead_Diaeresis = 0x01001257;
  const Key_Dead_Abovering = 0x01001258;
  const Key_Dead_Doubleacute = 0x01001259;
  const Key_Dead_Caron = 0x0100125a;
  const Key_Dead_Cedilla = 0x0100125b;
  const Key_Dead_Ogonek = 0x0100125c;
  const Key_Dead_Iota = 0x0100125d;
  const Key_Dead_Voiced_Sound = 0x0100125e;
  const Key_Dead_Semivoiced_Sound = 0x0100125f;
  const Key_Dead_Belowdot = 0x01001260;
  const Key_Dead_Hook = 0x01001261;
  const Key_Dead_Horn = 0x01001262;
  const Key_Dead_Stroke = 0x01001263;
  const Key_Dead_Abovecomma = 0x01001264;
  const Key_Dead_Abovereversedcomma = 0x01001265;
  const Key_Dead_Doublegrave = 0x01001266;
  const Key_Dead_Belowring = 0x01001267;
  const Key_Dead_Belowmacron = 0x01001268;
  const Key_Dead_Belowcircumflex = 0x01001269;
  const Key_Dead_Belowtilde = 0x0100126a;
  const Key_Dead_Belowbreve = 0x0100126b;
  const Key_Dead_Belowdiaeresis = 0x0100126c;
  const Key_Dead_Invertedbreve = 0x0100126d;
  const Key_Dead_Belowcomma = 0x0100126e;
  const Key_Dead_Currency = 0x0100126f;
  const Key_Dead_a = 0x01001280;
  const Key_Dead_A = 0x01001281;
  const Key_Dead_e = 0x01001282;
  const Key_Dead_E = 0x01001283;
  const Key_Dead_i = 0x01001284;
  const Key_Dead_I = 0x01001285;
  const Key_Dead_o = 0x01001286;
  const Key_Dead_O = 0x01001287;
  const Key_Dead_u = 0x01001288;
  const Key_Dead_U = 0x01001289;
  const Key_Dead_Small_Schwa = 0x0100128a;
  const Key_Dead_Capital_Schwa = 0x0100128b;
  const Key_Dead_Greek = 0x0100128c;
  const Key_Dead_Lowline = 0x01001290;
  const Key_Dead_Aboveverticalline = 0x01001291;
  const Key_Dead_Belowverticalline = 0x01001292;
  const Key_Dead_Longsolidusoverlay = 0x01001293;
  const Key_Back = 0x01000061;
  const Key_Forward = 0x01000062;
  const Key_Stop = 0x01000063;
  const Key_Refresh = 0x01000064;
  const Key_VolumeDown = 0x01000070;
  const Key_VolumeMute = 0x01000071;
  const Key_VolumeUp = 0x01000072;
  const Key_BassBoost = 0x01000073;
  const Key_BassUp = 0x01000074;
  const Key_BassDown = 0x01000075;
  const Key_TrebleUp = 0x01000076;
  const Key_TrebleDown = 0x01000077;
  const Key_MediaPlay = 0x01000080;
  const Key_MediaStop = 0x01000081;
  const Key_MediaPrevious = 0x01000082;
  const Key_MediaNext = 0x01000083;
  const Key_MediaRecord = 0x01000084;
  const Key_MediaPause = 0x1000085;
  const Key_MediaTogglePlayPause = 0x1000086;
  const Key_HomePage = 0x01000090;
  const Key_Favorites = 0x01000091;
  const Key_Search = 0x01000092;
  const Key_Standby = 0x01000093;
  const Key_OpenUrl = 0x01000094;
  const Key_LaunchMail = 0x010000a0;
  const Key_LaunchMedia = 0x010000a1;
  const Key_Launch0 = 0x010000a2;
  const Key_Launch1 = 0x010000a3;
  const Key_Launch2 = 0x010000a4;
  const Key_Launch3 = 0x010000a5;
  const Key_Launch4 = 0x010000a6;
  const Key_Launch5 = 0x010000a7;
  const Key_Launch6 = 0x010000a8;
  const Key_Launch7 = 0x010000a9;
  const Key_Launch8 = 0x010000aa;
  const Key_Launch9 = 0x010000ab;
  const Key_LaunchA = 0x010000ac;
  const Key_LaunchB = 0x010000ad;
  const Key_LaunchC = 0x010000ae;
  const Key_LaunchD = 0x010000af;
  const Key_LaunchE = 0x010000b0;
  const Key_LaunchF = 0x010000b1;
  const Key_LaunchG = 0x0100010e;
  const Key_LaunchH = 0x0100010f;
  const Key_MonBrightnessUp = 0x010000b2;
  const Key_MonBrightnessDown = 0x010000b3;
  const Key_KeyboardLightOnOff = 0x010000b4;
  const Key_KeyboardBrightnessUp = 0x010000b5;
  const Key_KeyboardBrightnessDown = 0x010000b6;
  const Key_PowerOff = 0x010000b7;
  const Key_WakeUp = 0x010000b8;
  const Key_Eject = 0x010000b9;
  const Key_ScreenSaver = 0x010000ba;
  const Key_WWW = 0x010000bb;
  const Key_Memo = 0x010000bc;
  const Key_LightBulb = 0x010000bd;
  const Key_Shop = 0x010000be;
  const Key_History = 0x010000bf;
  const Key_AddFavorite = 0x010000c0;
  const Key_HotLinks = 0x010000c1;
  const Key_BrightnessAdjust = 0x010000c2;
  const Key_Finance = 0x010000c3;
  const Key_Community = 0x010000c4;
  const Key_AudioRewind = 0x010000c5;
  const Key_BackForward = 0x010000c6;
  const Key_ApplicationLeft = 0x010000c7;
  const Key_ApplicationRight = 0x010000c8;
  const Key_Book = 0x010000c9;
  const Key_CD = 0x010000ca;
  const Key_Calculator = 0x010000cb;
  const Key_ToDoList = 0x010000cc;
  const Key_ClearGrab = 0x010000cd;
  const Key_Close = 0x010000ce;
  const Key_Copy = 0x010000cf;
  const Key_Cut = 0x010000d0;
  const Key_Display = 0x010000d1;
  const Key_DOS = 0x010000d2;
  const Key_Documents = 0x010000d3;
  const Key_Excel = 0x010000d4;
  const Key_Explorer = 0x010000d5;
  const Key_Game = 0x010000d6;
  const Key_Go = 0x010000d7;
  const Key_iTouch = 0x010000d8;
  const Key_LogOff = 0x010000d9;
  const Key_Market = 0x010000da;
  const Key_Meeting = 0x010000db;
  const Key_MenuKB = 0x010000dc;
  const Key_MenuPB = 0x010000dd;
  const Key_MySites = 0x010000de;
  const Key_News = 0x010000df;
  const Key_OfficeHome = 0x010000e0;
  const Key_Option = 0x010000e1;
  const Key_Paste = 0x010000e2;
  const Key_Phone = 0x010000e3;
  const Key_Calendar = 0x010000e4;
  const Key_Reply = 0x010000e5;
  const Key_Reload = 0x010000e6;
  const Key_RotateWindows = 0x010000e7;
  const Key_RotationPB = 0x010000e8;
  const Key_RotationKB = 0x010000e9;
  const Key_Save = 0x010000ea;
  const Key_Send = 0x010000eb;
  const Key_Spell = 0x010000ec;
  const Key_SplitScreen = 0x010000ed;
  const Key_Support = 0x010000ee;
  const Key_TaskPane = 0x010000ef;
  const Key_Terminal = 0x010000f0;
  const Key_Tools = 0x010000f1;
  const Key_Travel = 0x010000f2;
  const Key_Video = 0x010000f3;
  const Key_Word = 0x010000f4;
  const Key_Xfer = 0x010000f5;
  const Key_ZoomIn = 0x010000f6;
  const Key_ZoomOut = 0x010000f7;
  const Key_Away = 0x010000f8;
  const Key_Messenger = 0x010000f9;
  const Key_WebCam = 0x010000fa;
  const Key_MailForward = 0x010000fb;
  const Key_Pictures = 0x010000fc;
  const Key_Music = 0x010000fd;
  const Key_Battery = 0x010000fe;
  const Key_Bluetooth = 0x010000ff;
  const Key_WLAN = 0x01000100;
  const Key_UWB = 0x01000101;
  const Key_AudioForward = 0x01000102;
  const Key_AudioRepeat = 0x01000103;
  const Key_AudioRandomPlay = 0x01000104;
  const Key_Subtitle = 0x01000105;
  const Key_AudioCycleTrack = 0x01000106;
  const Key_Time = 0x01000107;
  const Key_Hibernate = 0x01000108;
  const Key_View = 0x01000109;
  const Key_TopMenu = 0x0100010a;
  const Key_PowerDown = 0x0100010b;
  const Key_Suspend = 0x0100010c;
  const Key_ContrastAdjust = 0x0100010d;
  const Key_TouchpadToggle = 0x01000110;
  const Key_TouchpadOn = 0x01000111;
  const Key_TouchpadOff = 0x01000112;
  const Key_MicMute = 0x01000113;
  const Key_Red = 0x01000114;
  const Key_Green = 0x01000115;
  const Key_Yellow = 0x01000116;
  const Key_Blue = 0x01000117;
  const Key_ChannelUp = 0x01000118;
  const Key_ChannelDown = 0x01000119;
  const Key_Guide = 0x0100011a;
  const Key_Info = 0x0100011b;
  const Key_Settings = 0x0100011c;
  const Key_MicVolumeUp = 0x0100011d;
  const Key_MicVolumeDown = 0x0100011e;
  const Key_New = 0x01000120;
  const Key_Open = 0x01000121;
  const Key_Find = 0x01000122;
  const Key_Undo = 0x01000123;
  const Key_Redo = 0x01000124;
  const Key_MediaLast = 0x0100ffff;
  const Key_unknown = 0x01ffffff;
  const Key_Call = 0x01100004;
  const Key_Camera = 0x01100020;
  const Key_CameraFocus = 0x01100021;
  const Key_Context1 = 0x01100000;
  const Key_Context2 = 0x01100001;
  const Key_Context3 = 0x01100002;
  const Key_Context4 = 0x01100003;
  const Key_Flip = 0x01100006;
  const Key_Hangup = 0x01100005;
  const Key_No = 0x01010002;
  const Key_Select = 0x01010000;
  const Key_Yes = 0x01010001;
  const Key_ToggleCallHangup = 0x01100007;
  const Key_VoiceDial = 0x01100008;
  const Key_LastNumberRedial = 0x01100009;
  const Key_Execute = 0x01020003;
  const Key_Printer = 0x01020002;
  const Key_Play = 0x01020005;
  const Key_Sleep = 0x01020004;
  const Key_Zoom = 0x01020006;
  const Key_Exit = 0x0102000a;
  const Key_Cancel = 0x01020001;

  export enum KeyCode {
    Key_Home,
    Key_End,
    Key_Left,
    Key_Up,
    Key_Right,
    Key_Down,
    Key_PageUp,
    Key_PageDown
  }

  export enum MouseEventButtons {
    LeftButton,
    RightButton,
    NoButton
  }

  export enum MouseButton {
    NoButton,
    AllButtons,
    LeftButton,
    RightButton,
    MidButton,
    MiddleButton,
    BackButton,
    XButton1,
    ExtraButton1,
    ForwardButton,
    XButton2,
    ExtraButton2,
    TaskButton,
    ExtraButton3,
    ExtraButton4,
    ExtraButton5,
    ExtraButton6,
    ExtraButton7,
    ExtraButton8,
    ExtraButton9,
    ExtraButton10,
    ExtraButton11,
    ExtraButton12,
    ExtraButton13,
    ExtraButton14,
    ExtraButton15,
    ExtraButton16,
    ExtraButton17,
    ExtraButton18,
    ExtraButton19,
    ExtraButton20,
    ExtraButton21,
    ExtraButton22,
    ExtraButton23,
    ExtraButton24
  }

  export class MouseEvent {
    button: Qt.MouseEventButtons;
  }

  const NoModifier = 0x00000000;
  const ShiftModifier = 0x02000000;
  const ControlModifier = 0x04000000;
  const AltModifier = 0x08000000;
  const MetaModifier = 0x10000000;
  const KeypadModifier = 0x20000000;
  const GroupSwitchModifier = 0x40000000;

  export enum KeyboardModifers {
    NoModifier,
    ShiftModifier,
    ControlModifier,
    AltModifier,
    MetaModifier,
    KeypadModifier,
    GroupSwitchModifier
  }

  export enum ComponentStatus {
    Null,
    Ready,
    Loading,
    Error
  }
}

declare class KeyEvent {
  accepted: boolean;
  count: number;
  isAutoRepeat: boolean;
  key: Qt.KeyCode;
  modifiers: number;
  nativeScanCode: number;
  text: string;
}
declare class Item {
  focus: boolean;
  x: number;
  y: number;
  height: number;
}

declare class Flickable extends Item {
  atXBeginning: boolean;
  atXEnd: boolean;
  atYBeginning: boolean;
  atYEnd: boolean;
  bottomMargin: number;
  contentHeight: number;
  contentItem: Item;
  contentWidth: number;
  contentX: number;
  contentY: number;
  dragging: boolean;
  draggingHorizontally: boolean;
  draggingVertically: boolean;
  flickDeceleration: number;
  flicking: boolean;
  flickingHorizontally: boolean;
  flickingVertically: boolean;
  horizontalOvershoot: number;
  horizontalVelocity: number;
  interactive: boolean;
  leftMargin: number;
  maximumFlickVelocity: number;
  moving: boolean;
  movingHorizontally: boolean;
  movingVertically: boolean;
  originX: number;
  originY: number;
  pixelAligned: boolean;
  pressDelay: number;
  rightMargin: number;
  synchronousDrag: boolean;
  topMargin: number;
  verticalOvershoot: number;
  verticalVelocity: number;
}

declare enum ScrollBarPolicy {
  AlwaysOn,
  AlwaysOff,
  AsNeeded
}

declare class ScrollBar extends Item {
  policy: ScrollBarPolicy;
  position: number;
  setPosition(pos: number): void;
  increase(): void;
  decrease(): void;
}

declare class ListView<T extends Item> extends Flickable {
  currentIndex: number;
  currentItem: T;
  itemAtIndex(index: number): Item;

  positionViewAtEnd(): void;
  forceLayout(): void;
  positionViewAtBeginning(): void;
}

declare class Popup {
  close(): void;
  open(): void;
}

declare class Menu extends Popup {}

declare class TextArea {
  text: string;
  cursorPosition: number;
  selectionEnd: number;
  insert(position: number, text: string): void;
  clear(): void;
}

declare class TextField {
  text: string;
}

declare class QKeyEvent {
  modifiers: Qt.KeyboardModifers;
}

declare class Component {
  status: Qt.ComponentStatus;
}

declare class QPoint {
  x: number;
  y: number;
}

declare class Page {
  mapFromItem(item: Item, x: number, y: number): QPoint;
  height: number;
}

declare class Repeater extends Item {
  itemAt(index: number): Item;
}
