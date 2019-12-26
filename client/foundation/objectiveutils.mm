#include "objectiveutils.h"
#include <QWidget>
#include <QColor>
#import <Foundation/Foundation.h>

ObjectiveUtils::ObjectiveUtils(){
};

#ifdef Q_OS_IOS
#import <UIKit/UIKit.h>
#import <UserNotifications/UserNotifications.h>

void ObjectiveUtils::set_status_bar_color(QColor color) {
       UIApplication *app =  [UIApplication sharedApplication];
       app.windows.firstObject.rootViewController.view.backgroundColor
           =  [UIColor colorWithRed:color.redF() green:color.greenF() blue:color.blueF()  alpha:1.0];
}

void ObjectiveUtils::request_notifications()
{
  UNUserNotificationCenter* center = [UNUserNotificationCenter currentNotificationCenter];
  [center requestAuthorizationWithOptions:
              (UNAuthorizationOptionAlert +
          UNAuthorizationOptionSound)
                        completionHandler:^(BOOL granted, NSError * _Nullable error) {
                        }];
}
#endif
