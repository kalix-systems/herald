#include "objectiveutils.h"
#include <QWidget>
#include <QColor>
#import <UserNotifications/UserNotifications.h>
#import <Foundation/Foundation.h>

ObjectiveUtils::ObjectiveUtils(){
};

#ifdef Q_OS_IOS
#import <UIKit/UIKit.h>
#import <MobileCoreServices/MobileCoreServices.h>


@interface FileDialogHelper :NSObject<UIDocumentPickerDelegate> {
  id delegate;
  NSArray<NSString *> * filenames;
}

- (void)openDocumentPicker;

- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls;

- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller;

@end

@implementation FileDialogHelper

- (void) openDocumentPicker {
    auto *rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    UIDocumentPickerViewController* dp = [[UIDocumentPickerViewController alloc] initWithDocumentTypes:@[(NSString *)kUTTypePDF] inMode:UIDocumentPickerModeImport];
    [rvc presentViewController:dp animated: YES completion: nil];
}

- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls {
    
    
}

- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller {
    
}

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

QString ObjectiveUtils::launch_file_picker()
{
    FileDialogHelper* dialog = [[FileDialogHelper alloc] init];
    [dialog openDocumentPicker];
    return QString("");
}

@end


#endif
