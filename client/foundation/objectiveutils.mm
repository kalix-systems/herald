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


@interface FileDialogHelper :NSObject<UIDocumentPickerDelegate,UIImagePickerControllerDelegate,UINavigationControllerDelegate>{
    ObjectiveUtils* _util;
}
- (void)setUtils:(ObjectiveUtils *) util;
- (void)openDocumentPicker;
- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls;
- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller;
- (void)imagePickerController:(UIImagePickerController *)picker didFinishPickingMediaWithInfo:(NSDictionary<UIImagePickerControllerInfoKey, id> *)info;
- (void)imagePickerControllerDidCancel:(UIImagePickerController *)picker;


@end

@implementation FileDialogHelper

- (void)imagePickerController:(UIImagePickerController *)picker didFinishPickingMediaWithInfo:(NSDictionary<UIImagePickerControllerInfoKey, id> *)info {
    const char * fname = [[info[UIImagePickerControllerImageURL] absoluteString] UTF8String];
    emit _util->fileChosen(fname);
    auto* rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    [rvc dismissViewControllerAnimated:YES completion:nil];
    [self release];
}

- (void)imagePickerControllerDidCancel:(UIImagePickerController *)picker {
    auto* rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    [rvc dismissViewControllerAnimated:YES completion:nil];
    [self release];
}

- (void) openDocumentPicker {
    auto *rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    UIDocumentPickerViewController* dp = [[UIDocumentPickerViewController alloc] initWithDocumentTypes:@[@"public.item"] inMode:UIDocumentPickerModeImport];
    dp.allowsMultipleSelection = NO;
    dp.delegate = self;
    [rvc presentViewController:dp animated: YES completion: nil];
}

- (void)documentPicker:(UIDocumentPickerViewController *)controller didPickDocumentsAtURLs:(NSArray<NSURL *> *)urls {
    if (urls.count > 0) {
        const char * fname = [[urls[0] absoluteString] UTF8String];
        emit _util->fileChosen(QString(fname));
    }
    [self release];
}

- (void)documentPickerWasCancelled:(UIDocumentPickerViewController *)controller {
    [self release];
 }


- (void)setUtils:(ObjectiveUtils *) util {
   _util = util;
}

- (void) openImageDialog {
    
    auto* rvc =  [[UIApplication sharedApplication].keyWindow rootViewController];
    auto* alert = [UIAlertController  alertControllerWithTitle:nil message:nil  preferredStyle:UIAlertControllerStyleActionSheet];
    
    [alert addAction: [UIAlertAction actionWithTitle: @"Cancel" style:UIAlertActionStyleCancel handler:^(UIAlertAction* action){
        (void)action;
    }]];
    
    [alert addAction: [UIAlertAction actionWithTitle: @"Select From Gallery" style:UIAlertActionStyleDefault  handler:^(UIAlertAction* action){
        auto picker = [[UIImagePickerController alloc] init];
        picker.sourceType = UIImagePickerControllerSourceTypeSavedPhotosAlbum;
        picker.delegate = self;
        [rvc presentViewController:picker animated: YES completion: nil];
    }]];
    
    [alert addAction:  [UIAlertAction actionWithTitle: @"Use Camera"  style:UIAlertActionStyleDefault handler:^(UIAlertAction* action){
        auto picker = [[UIImagePickerController alloc] init];
        if([UIImagePickerController isSourceTypeAvailable: UIImagePickerControllerSourceTypeCamera]) {
        picker.sourceType = UIImagePickerControllerSourceTypeCamera;
        picker.delegate = self;
        [rvc presentViewController:picker animated: YES completion: nil];
        }
    }]];
    
    [rvc presentViewController:alert animated: YES completion: nil];
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

void ObjectiveUtils::launch_file_picker()
{
    FileDialogHelper* dialog = [[FileDialogHelper alloc] init];
    [dialog setUtils: this];
    [dialog openDocumentPicker];
}

void ObjectiveUtils::launch_camera_dialog(){
    FileDialogHelper* dialog = [[FileDialogHelper alloc] init];
    [dialog setUtils: this];
    [dialog openImageDialog];
}


@end


#endif
