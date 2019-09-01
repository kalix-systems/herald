#ifndef QMLCONSTANTS_H
#define QMLCONSTANTS_H

#include <QObject>

/// This class contains enumerations
/// of different constants and status
/// s.t. they are available in a sane form
/// from qml.
class QmlConstants: public QObject
{
  Q_OBJECT
public:
  QmlConstants();

  enum AckTypes
  {
    /// No ack from any third party
    NoAck = 0,
    /// Received by the server, and made it to the user
    ReceivedAck = 1,
    /// Received by the recipient
    RecpientReadAck = 2,
    /// The message has timedout.
    Timeout = 3,
    /// we did not write this message
    Inbound = 4,
    /// The user has read receipts turned off
    AckTerminal = 5,
  };
  Q_ENUM(AckTypes)
};

#endif // QMLCONSTANTS_H
