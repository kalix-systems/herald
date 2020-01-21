#ifndef CONVERSATIONMAP_H
#define CONVERSATIONMAP_H
#include "Bindings.h"
#include <QHash>


/// If you are reading this file please read this preface before hand.
/// ConversationContent objects are just thin routing wrappers on top
/// of a messages and members model. as such this is not the massive
/// memory leak which it appears to be. the actual content of these
/// conversations is treasured away in a database.
class ConversationMap: public QObject {
  Q_OBJECT
public:
        ConversationMap(){}
        /// get(msgid), attempts to get a pointer to a conversation
        /// with Id msgid, if it does not exist, it is allocated and inserted
        Q_INVOKABLE QVariant get(const QByteArray msgId)  {
          auto iter = conversationHash.find(msgId);
          if(iter == conversationHash.end()) {
            // conversation does not exist
            auto conv = new ConversationContent();
            conv->setConversationId(msgId);
            conversationHash.insert(msgId, conv);
            return QVariant::fromValue(conv);
          } else {
            // conversation exists
            return QVariant::fromValue(iter.value());
          }
        }
private:
     QHash<QByteArray,ConversationContent*> conversationHash;

};

#endif // CONVERSATIONMAP_H
