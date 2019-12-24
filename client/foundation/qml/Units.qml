
/*
 *   Copyright 2015 Marco Martin <mart@kde.org>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU Library General Public License for more details
 *
 *   You should have received a copy of the GNU Library General Public
 *   License along with this program; if not, write to the
 *   Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */
import QtQuick 2.13
import QtQuick.Window 2.13


/**
 * A set of values to define semantically sizes and durations
 * @inherit QtQuick.QtObject
 */
QtObject {


    /**
     * The ratio between physical and device-independent pixels. This value does not depend on the \
     * size of the configured font. If you want to take font sizes into account when scaling elements,
     * use theme.mSize(theme.defaultFont), units.smallSpacing and units.largeSpacing.
     * The devicePixelRatio follows the definition of "device independent pixel" by Microsoft.
     */

    //readonly property real devicePixelRatio: Screen.devicePixelRatio
    // 25.4 = mm per in
    // 160 =
    readonly property real deviceDotsPerInch: (Screen.pixelDensity * 25.4) / 160

    function dp(dips) {
        return dips * deviceDotsPerInch * 1.25
    }


    /**
     * units.longDuration should be used for longer, screen-covering animations, for opening and
     * closing of dialogs and other "not too small" animations
     */
    property int longDuration: 250


    /**
     * units.shortDuration should be used for short animations, such as accentuating a UI event,
     * hover events, etc..
     */
    property int shortDuration: 150


    /**
     * metrics used by the default font
     */
    // TODO decide whether to use this vs defining rounding behavior ourselves
    //    property variant fontMetrics: TextMetrics {
    //        text: "M"
    //        function roundedIconSize(size) {
    //            if (size < 16) {
    //                return size
    //            } else if (size < 22) {
    //                return 16
    //            } else if (size < 32) {
    //                return 22
    //            } else if (size < 48) {
    //                return 32
    //            } else if (size < 64) {
    //                return 48
    //            } else {
    //                return size
    //            }
    //        }
    //    }
}
