package org.qtproject.example.notification;

import  org.qtproject.qt5.android.QtNative;

import  android.app.Notification;
import  android.app.NotificationManager;
import  android.app.NotificationChannel;
import  android.content.Context;

public class NotificationBuilder extends org.qtproject.qt5.android.bindings.QtActivity
{
    private static NotificationManager m_notificationManager;
    private static Notification.Builder m_builder;
    private static NotificationChannel m_channel;
    private static NotificationBuilder m_instance;
    private static int id_ct;

    public NotificationBuilder()
    {
        id_ct = 0;
        m_instance = this;
        String channelId = "herald messages";
        m_channel = new NotificationChannel(channelId,channelId, NotificationManager.IMPORTANCE_DEFAULT);
    }

    public static void notify(String s)
    {

        id_ct += 1;

        if (m_notificationManager == null) {
            m_notificationManager = (NotificationManager)m_instance.getSystemService(Context.NOTIFICATION_SERVICE);
            m_notificationManager.createNotificationChannel(m_channel);
            m_builder = new Notification.Builder(m_instance);
            m_builder.setSmallIcon(android.R.drawable.btn_star);
            m_builder.setContentTitle("A message from Qt!");
        }

        m_builder.setContentText(s);
        String channelId = "herald messages";
        m_builder.setChannelId(channelId);
        m_notificationManager.notify(id_ct, m_builder.build());
    }
}
