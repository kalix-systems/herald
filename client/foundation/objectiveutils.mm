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


@interface FileDialogHelper :NSObject<UIDocumentPickerDelegate>{
    ObjectiveUtils* _util;
}
- (void)setUtils:(ObjectiveUtils *) util;
- (void)openDocumentPicker;
- (void)openCameraView;
- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls;
- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller;

@end

@implementation FileDialogHelper

- (void) openDocumentPicker {
    auto *rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    UIDocumentPickerViewController* dp = [[UIDocumentPickerViewController alloc] initWithDocumentTypes:@[(NSString *)kUTTypePDF] inMode:UIDocumentPickerModeImport];
    dp.delegate = self;
    [rvc presentViewController:dp animated: YES completion: nil];
}

- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls {
    (void)controller;
    const char* fname = [urls[0] UTF8String];
    emit _util->fileChosen(QString(fname));
    [self release];
}

- (void)setUtils:(ObjectiveUtils *) util {
   _util = util;
}

- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller {
  emit _util->fileChosen(QString(""));
  [self release];
}

- (void) openCameraView {
    
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
    [dialog setUtils: this];
    [dialog openDocumentPicker];
    return QString();
}

@end


#endif
