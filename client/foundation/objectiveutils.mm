#include "objectiveutils.h"
#include <QWidget>
#import <Foundation/Foundation.h>

ObjectiveUtils::ObjectiveUtils(){
};

#ifdef Q_OS_IOS
#import <UIKit/UIKit.h>
void ObjectiveUtils::set_navbar_color() {
       UIApplication *app =  [UIApplication sharedApplication];
       app.windows.firstObject.rootViewController.view.backgroundColor
           =  [UIColor colorWithRed:0.23  green:0.24   blue:0.25   alpha:1.0];
}
#endif
