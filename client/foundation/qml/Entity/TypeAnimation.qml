import QtQuick 2.14

SequentialAnimation {
    id: anim
    loops: Animation.Infinite
    ParallelAnimation {
        PropertyAnimation {
            target: rect1
            property: "color"
            to: typeColorAnim
        }
        PropertyAnimation {
            target: rect1
            property: "height"
            to: bigType
        }
        PropertyAnimation {
            target: rect1
            property: "width"
            to: bigType
        }
    }
    PauseAnimation {
        duration: 80
    }

    ParallelAnimation {
        PropertyAnimation {
            target: rect1
            property: "color"
            to: typeColor
        }
        PropertyAnimation {
            target: rect1
            property: "height"
            to: typeSize
        }
        PropertyAnimation {
            target: rect1
            property: "width"
            to: typeSize
        }
    }

    PauseAnimation {
        duration: 80
    }

    ParallelAnimation {
        PropertyAnimation {
            target: rect2
            property: "color"
            to: typeColorAnim
        }
        PropertyAnimation {
            target: rect2
            property: "height"
            to: bigType
        }
        PropertyAnimation {
            target: rect2
            property: "width"
            to: bigType
        }
    }
    PauseAnimation {
        duration: 80
    }

    ParallelAnimation {
        PropertyAnimation {
            target: rect2
            property: "color"
            to: typeColor
        }
        PropertyAnimation {
            target: rect2
            property: "height"
            to: typeSize
        }
        PropertyAnimation {
            target: rect2
            property: "width"
            to: typeSize
        }
    }

    PauseAnimation {
        duration: 80
    }

    ParallelAnimation {
        PropertyAnimation {
            target: rect3
            property: "color"
            to: typeColorAnim
        }
        PropertyAnimation {
            target: rect3
            property: "height"
            to: bigType
        }
        PropertyAnimation {
            target: rect3
            property: "width"
            to: bigType
        }
    }
    PauseAnimation {
        duration: 80
    }

    ParallelAnimation {
        PropertyAnimation {
            target: rect3
            property: "color"
            to: typeColor
        }
        PropertyAnimation {
            target: rect3
            property: "height"
            to: typeSize
        }
        PropertyAnimation {
            target: rect3
            property: "width"
            to: typeSize
        }
    }

    PauseAnimation {
        duration: 140
    }
}
