#ifndef QQQMLCLIPBOARD_H
#define QQQMLCLIPBOARD_H

#include <QObject>
#include <QClipboard>
#include <QGuiApplication>

class QqmlClipBoard : public QObject
{
  Q_OBJECT
public:
  QqmlClipBoard() {

  }
  QClipboard* m_clipboard = QGuiApplication::clipboard();
  Q_INVOKABLE void copyToCLipboard(QString text) {
    m_clipboard->setText(text);
  }

};

#endif // QQQMLCLIPBOARD_H
