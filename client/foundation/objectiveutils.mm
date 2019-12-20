#include "objectiveutils.h"
#include <QWidget>
#include <QColor>
#import <Foundation/Foundation.h>

ObjectiveUtils::ObjectiveUtils(){
};

#ifdef Q_OS_IOS
#import <UIKit/UIKit.h>
void ObjectiveUtils::set_status_bar_color(QColor color) {
       UIApplication *app =  [UIApplication sharedApplication];
       app.windows.firstObject.rootViewController.view.backgroundColor
           =  [UIColor colorWithRed:color.redF() green:color.greenF() blue:color.blueF()  alpha:1.0];

}

void ObjectiveUtils::send_notification(QString content) {

}

#endif
