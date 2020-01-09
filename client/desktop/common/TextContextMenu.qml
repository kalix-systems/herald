import QtQuick 2.4

import Qt.labs.platform 1.0
import LibHerald 1.0
import "." as Common

MouseArea {
    id: menuMouse

    property var parentText
    anchors.fill: parent
    acceptedButtons: Qt.RightButton
    hoverEnabled: true
    cursorShape: Qt.IBeamCursor
    property int selectStart
    property int selectEnd
    property int curPos
    onClicked: {
        selectStart = parentText.selectionStart
        selectEnd = parentText.selectionEnd
        curPos = parentText.cursorPosition
        contextMenu.open()
        parentText.cursorPosition = curPos
        parentText.select(selectStart, selectEnd)
    }
    onPressAndHold: {
        if (mouse.source === Qt.MouseEventNotSynthesized) {
            selectStart = parentText.selectionStart
            selectEnd = parentText.selectionEnd
            curPos = parentText.cursorPosition
            contextMenu.open()
            parentText.cursorPosition = curPos
            parentText.select(selectStart, selectEnd)
        }
    }

    Menu {
        id: contextMenu
        MenuItem {
            text: "✂ Cut"
            enabled: selectStart < selectEnd
            shortcut: "Ctrl+X"
            onTriggered: {
                menuMouse.parentText.cut()
            }
        }
        MenuItem {
            text: "⎘ Copy"
            shortcut: "Ctrl+C"
            enabled: selectStart < selectEnd
            onTriggered: {
                menuMouse.parentText.copy()
            }
        }
        MenuItem {
            text: "Paste"
            shortcut: "Ctrl+V"
            enabled: menuMouse.parentText.canPaste
            onTriggered: {
                menuMouse.parentText.paste()
            }
        }
        MenuItem {
            text: "Select all"
            shortcut: "Ctrl+A"
            enabled: parentText.text.length > 0
            onTriggered: {
                menuMouse.parentText.selectAll()
            }
        }

        MenuItem {
            text: "↰ Undo"
            shortcut: "Ctrl+Z"
            enabled: menuMouse.parentText.canUndo
            onTriggered: {
                menuMouse.parentText.undo()
            }
        }
        MenuItem {
            text: "↱ Redo"
            shortcut: "Ctrl+Shift+Z"
            enabled: menuMouse.parentText.canRedo
            onTriggered: {
                menuMouse.parentText.redo()
            }
        }
    }
}
