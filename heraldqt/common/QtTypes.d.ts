declare namespace Qt {
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

declare class Popup {
  close(): void;
}

declare class TextArea {
  text: string;
  cursorPosition: number;
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
