#ifndef QMLCONSTANTS_H
#define QMLCONSTANTS_H

#include <QObject>

/// This class contains enumerations
/// of different constants and status
/// s.t. they are available in a sane form
/// from qml.
class QmlConstants : public QObject {
  Q_OBJECT
public:
  QmlConstants();

  enum AckTypes {
    /// No ack from any third party
    NoAck,
    /// Received by the server, and made it to the user
    ReceivedAck,
    /// Received by the recipient
    RecpientReadAck,
    /// The message has timedout.
    Timeout,
    /// we did not write this message
    Inbound,
    /// The user has read receipts turned off
    AckTerminal,
  };
  Q_ENUM(AckTypes)
};

#endif // QMLCONSTANTS_H
