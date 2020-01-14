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
  NSString* filename;
}

- (void)openDocumentPicker;
- (void)openCameraView;
- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls;
- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller;

@end

@implementation FileDialogHelper

- (void) openDocumentPicker {
    auto *rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    UIDocumentPickerViewController* dp = [[UIDocumentPickerViewController alloc] initWithDocumentTypes:@[(NSString *)kUTTypeText,(NSString *)kUTTypePDF,(NSString *)kUTTypeAudiovisualContent, (NSString *)kUTTypeImage] inMode:UIDocumentPickerModeImport];
    [rvc presentViewController:dp animated: YES completion: nil];
}

- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls {
    for(NSURL* url in urls) {
        NSLog(@"%@", url);
    }
    // just return the first item of the urls array for simplicity
    filename = urls[0].absoluteString;
}

- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller {
    filename = @"";
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
    [dialog openDocumentPicker];
    return QString([[dialog filename] UTF8String]);
}

@end


#endif
