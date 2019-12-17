#include "iosutils.h"
#include <QWidget>
#import <Foundation/Foundation.h>
#include <Cocoa/Cocoa.h>


void IosUtils::set_window_color(WId winId){
  NSView* view = (NSView*)winId;
  NSWindow* window = [view window];
  window.titlebarAppearsTransparent = YES;
  window.backgroundColor = [NSColor colorWithRed:0.22 green:0.23 blue:0.24 alpha:1.];
}
