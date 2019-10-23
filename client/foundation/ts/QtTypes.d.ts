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

  class MouseEvent {
    button: MouseEventButtons;
  }

  enum MouseEventButtons {
    LeftButton,
    RightButton,
    NoButton
  }

  enum MouseButton {
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

  const NoModifier = 0x00000000;
  const ShiftModifier = 0x02000000;
  const ControlModifier = 0x04000000;
  const AltModifier = 0x08000000;
  const MetaModifier = 0x10000000;
  const KeypadModifier = 0x20000000;
  const GroupSwitchModifier = 0x40000000;

  enum KeyboardModifers {
    NoModifier,
    ShiftModifier,
    ControlModifier,
    AltModifier,
    MetaModifier,
    KeypadModifier,
    GroupSwitchModifier
  }

  enum ComponentStatus {
    Null,
    Ready,
    Loading,
    Error
  }
}

declare class ListView<T extends Item> {
  currentIndex: number;
  currentItem: T;
}

declare class Item {
  focus: boolean;
}

declare class Popup {
  close(): void;
  open(): void;
}

declare class Menu extends Popup { }

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
