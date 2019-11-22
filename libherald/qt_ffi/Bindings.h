/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QAbstractItemModel>
#include <QtCore/QObject>

class Attachments;
class Config;
class ConversationBuilder;
class Conversations;
class Errors;
class Herald;
class Members;
class MessageBuilder;
class MessageSearch;
class Messages;
class Users;
class UsersSearch;
class Utils;

class Attachments : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(
      QByteArray msgId READ msgId WRITE setMsgId NOTIFY msgIdChanged FINAL)
  explicit Attachments(bool owned, QObject *parent);

public:
  explicit Attachments(QObject *parent = nullptr);
  ~Attachments() override;
  QByteArray msgId() const;
  void setMsgId(const QByteArray &v);
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  Q_INVOKABLE QString attachmentPath(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void msgIdChanged();
};

class Config : public QObject {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(quint32 color READ color WRITE setColor NOTIFY colorChanged FINAL)
  Q_PROPERTY(quint32 colorscheme READ colorscheme WRITE setColorscheme NOTIFY
                 colorschemeChanged FINAL)
  Q_PROPERTY(QString configId READ configId NOTIFY configIdChanged FINAL)
  Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged FINAL)
  Q_PROPERTY(QByteArray ntsConversationId READ ntsConversationId NOTIFY
                 ntsConversationIdChanged FINAL)
  Q_PROPERTY(QString profilePicture READ profilePicture WRITE setProfilePicture
                 NOTIFY profilePictureChanged FINAL)
  explicit Config(bool owned, QObject *parent);

public:
  explicit Config(QObject *parent = nullptr);
  ~Config() override;
  quint32 color() const;
  void setColor(quint32 v);
  quint32 colorscheme() const;
  void setColorscheme(quint32 v);
  QString configId() const;
  QString name() const;
  void setName(const QString &v);
  QByteArray ntsConversationId() const;
  QString profilePicture() const;
  void setProfilePicture(const QString &v);
Q_SIGNALS:
  void colorChanged();
  void colorschemeChanged();
  void configIdChanged();
  void nameChanged();
  void ntsConversationIdChanged();
  void profilePictureChanged();
};

class ConversationBuilder : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(
      QString picture READ picture WRITE setPicture NOTIFY pictureChanged FINAL)
  explicit ConversationBuilder(bool owned, QObject *parent);

public:
  explicit ConversationBuilder(QObject *parent = nullptr);
  ~ConversationBuilder() override;
  QString picture() const;
  void setPicture(const QString &v);
  Q_INVOKABLE bool addMember(const QString &user_id);
  Q_INVOKABLE void finalize();
  Q_INVOKABLE void removeLast();
  Q_INVOKABLE bool removeMemberById(const QString &user_id);
  Q_INVOKABLE bool removeMemberByIndex(quint64 index);
  Q_INVOKABLE void setTitle(const QString &title);
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  Q_INVOKABLE QString memberId(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void pictureChanged();
};

class Conversations : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(
      QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
  Q_PROPERTY(bool filterRegex READ filterRegex WRITE setFilterRegex NOTIFY
                 filterRegexChanged FINAL)
  explicit Conversations(bool owned, QObject *parent);

public:
  explicit Conversations(QObject *parent = nullptr);
  ~Conversations() override;
  QString filter() const;
  void setFilter(const QString &v);
  bool filterRegex() const;
  void setFilterRegex(bool v);
  Q_INVOKABLE void clearFilter();
  Q_INVOKABLE bool removeConversation(quint64 row_index);
  Q_INVOKABLE bool toggleFilterRegex();
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  bool setData(const QModelIndex &index, const QVariant &value,
               int role = Qt::EditRole) override;
  Q_INVOKABLE quint32 color(int row) const;
  Q_INVOKABLE bool setColor(int row, quint32 value);
  Q_INVOKABLE QByteArray conversationId(int row) const;
  Q_INVOKABLE quint8 expirationPeriod(int row) const;
  Q_INVOKABLE bool setExpirationPeriod(int row, quint8 value);
  Q_INVOKABLE bool matched(int row) const;
  Q_INVOKABLE bool muted(int row) const;
  Q_INVOKABLE bool setMuted(int row, bool value);
  Q_INVOKABLE bool pairwise(int row) const;
  Q_INVOKABLE QString picture(int row) const;
  Q_INVOKABLE bool setPicture(int row, const QString &value);
  Q_INVOKABLE QString title(int row) const;
  Q_INVOKABLE bool setTitle(int row, const QString &value);

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void filterChanged();
  void filterRegexChanged();
};

class Errors : public QObject {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(quint8 tryPoll READ tryPoll NOTIFY tryPollChanged FINAL)
  explicit Errors(bool owned, QObject *parent);

public:
  explicit Errors(QObject *parent = nullptr);
  ~Errors() override;
  quint8 tryPoll() const;
  Q_INVOKABLE QString nextError();
Q_SIGNALS:
  void tryPollChanged();
};

class Herald : public QObject {
  Q_OBJECT
  friend class Messages;

public:
  class Private;

private:
  Config *const m_config;
  ConversationBuilder *const m_conversationBuilder;
  Conversations *const m_conversations;
  Errors *const m_errors;
  MessageSearch *const m_messageSearch;
  Users *const m_users;
  UsersSearch *const m_usersSearch;
  Utils *const m_utils;
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(Config *config READ config NOTIFY configChanged FINAL)
  Q_PROPERTY(bool configInit READ configInit NOTIFY configInitChanged FINAL)
  Q_PROPERTY(bool connectionPending READ connectionPending NOTIFY
                 connectionPendingChanged FINAL)
  Q_PROPERTY(
      bool connectionUp READ connectionUp NOTIFY connectionUpChanged FINAL)
  Q_PROPERTY(ConversationBuilder *conversationBuilder READ conversationBuilder
                 NOTIFY conversationBuilderChanged FINAL)
  Q_PROPERTY(Conversations *conversations READ conversations NOTIFY
                 conversationsChanged FINAL)
  Q_PROPERTY(Errors *errors READ errors NOTIFY errorsChanged FINAL)
  Q_PROPERTY(MessageSearch *messageSearch READ messageSearch NOTIFY
                 messageSearchChanged FINAL)
  Q_PROPERTY(Users *users READ users NOTIFY usersChanged FINAL)
  Q_PROPERTY(
      UsersSearch *usersSearch READ usersSearch NOTIFY usersSearchChanged FINAL)
  Q_PROPERTY(Utils *utils READ utils NOTIFY utilsChanged FINAL)
  explicit Herald(bool owned, QObject *parent);

public:
  explicit Herald(QObject *parent = nullptr);
  ~Herald() override;
  const Config *config() const;
  Config *config();
  bool configInit() const;
  bool connectionPending() const;
  bool connectionUp() const;
  const ConversationBuilder *conversationBuilder() const;
  ConversationBuilder *conversationBuilder();
  const Conversations *conversations() const;
  Conversations *conversations();
  const Errors *errors() const;
  Errors *errors();
  const MessageSearch *messageSearch() const;
  MessageSearch *messageSearch();
  const Users *users() const;
  Users *users();
  const UsersSearch *usersSearch() const;
  UsersSearch *usersSearch();
  const Utils *utils() const;
  Utils *utils();
  Q_INVOKABLE bool login();
  Q_INVOKABLE void registerNewUser(const QString &user_id);
Q_SIGNALS:
  void configChanged();
  void configInitChanged();
  void connectionPendingChanged();
  void connectionUpChanged();
  void conversationBuilderChanged();
  void conversationsChanged();
  void errorsChanged();
  void messageSearchChanged();
  void usersChanged();
  void usersSearchChanged();
  void utilsChanged();
};

class Members : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(QByteArray conversationId READ conversationId WRITE
                 setConversationId NOTIFY conversationIdChanged FINAL)
  Q_PROPERTY(
      QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
  Q_PROPERTY(bool filterRegex READ filterRegex WRITE setFilterRegex NOTIFY
                 filterRegexChanged FINAL)
  explicit Members(bool owned, QObject *parent);

public:
  explicit Members(QObject *parent = nullptr);
  ~Members() override;
  QByteArray conversationId() const;
  void setConversationId(const QByteArray &v);
  QString filter() const;
  void setFilter(const QString &v);
  bool filterRegex() const;
  void setFilterRegex(bool v);
  Q_INVOKABLE bool addToConversation(const QString &id);
  Q_INVOKABLE bool removeFromConversationByIndex(quint64 row_index);
  Q_INVOKABLE bool toggleFilterRegex();
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  Q_INVOKABLE quint32 color(int row) const;
  Q_INVOKABLE bool matched(int row) const;
  Q_INVOKABLE QString name(int row) const;
  Q_INVOKABLE QByteArray pairwiseConversationId(int row) const;
  Q_INVOKABLE QString profilePicture(int row) const;
  Q_INVOKABLE quint8 status(int row) const;
  Q_INVOKABLE QString userId(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void conversationIdChanged();
  void filterChanged();
  void filterRegexChanged();
};

class MessageBuilder : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(QString body READ body WRITE setBody NOTIFY bodyChanged FINAL)
  Q_PROPERTY(bool isMediaMessage READ isMediaMessage NOTIFY
                 isMediaMessageChanged FINAL)
  Q_PROPERTY(bool isReply READ isReply NOTIFY isReplyChanged FINAL)
  Q_PROPERTY(QString opAuthor READ opAuthor NOTIFY opAuthorChanged FINAL)
  Q_PROPERTY(QString opBody READ opBody NOTIFY opBodyChanged FINAL)
  Q_PROPERTY(QVariant opHasAttachments READ opHasAttachments NOTIFY
                 opHasAttachmentsChanged FINAL)
  Q_PROPERTY(QByteArray opId READ opId NOTIFY opIdChanged FINAL)
  Q_PROPERTY(QVariant opTime READ opTime NOTIFY opTimeChanged FINAL)
  Q_PROPERTY(bool parseMarkdown READ parseMarkdown WRITE setParseMarkdown NOTIFY
                 parseMarkdownChanged FINAL)
  explicit MessageBuilder(bool owned, QObject *parent);

public:
  explicit MessageBuilder(QObject *parent = nullptr);
  ~MessageBuilder() override;
  QString body() const;
  void setBody(const QString &v);
  bool isMediaMessage() const;
  bool isReply() const;
  QString opAuthor() const;
  QString opBody() const;
  QVariant opHasAttachments() const;
  QByteArray opId() const;
  QVariant opTime() const;
  bool parseMarkdown() const;
  void setParseMarkdown(bool v);
  Q_INVOKABLE bool addAttachment(const QString &path);
  Q_INVOKABLE void clearReply();
  Q_INVOKABLE void finalize();
  Q_INVOKABLE bool removeAttachment(const QString &path);
  Q_INVOKABLE bool removeAttachmentByIndex(quint64 row_index);
  Q_INVOKABLE void removeLast();
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  Q_INVOKABLE QString attachmentPath(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void bodyChanged();
  void isMediaMessageChanged();
  void isReplyChanged();
  void opAuthorChanged();
  void opBodyChanged();
  void opHasAttachmentsChanged();
  void opIdChanged();
  void opTimeChanged();
  void parseMarkdownChanged();
};

class MessageSearch : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(QVariant regexSearch READ regexSearch WRITE setRegexSearch NOTIFY
                 regexSearchChanged FINAL)
  Q_PROPERTY(QString searchPattern READ searchPattern WRITE setSearchPattern
                 NOTIFY searchPatternChanged FINAL)
  explicit MessageSearch(bool owned, QObject *parent);

public:
  explicit MessageSearch(QObject *parent = nullptr);
  ~MessageSearch() override;
  QVariant regexSearch() const;
  void setRegexSearch(const QVariant &v);
  QString searchPattern() const;
  void setSearchPattern(const QString &v);
  Q_INVOKABLE void clearSearch();
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  Q_INVOKABLE QString author(int row) const;
  Q_INVOKABLE QString body(int row) const;
  Q_INVOKABLE QByteArray conversation(int row) const;
  Q_INVOKABLE QVariant conversationColor(int row) const;
  Q_INVOKABLE QVariant conversationPairwise(int row) const;
  Q_INVOKABLE QString conversationPicture(int row) const;
  Q_INVOKABLE QString conversationTitle(int row) const;
  Q_INVOKABLE QVariant has_attachments(int row) const;
  Q_INVOKABLE QByteArray msgId(int row) const;
  Q_INVOKABLE QVariant time(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void regexSearchChanged();
  void searchPatternChanged();
};

class Messages : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;

public:
  class Private;

private:
  MessageBuilder *const m_builder;
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(MessageBuilder *builder READ builder NOTIFY builderChanged FINAL)
  Q_PROPERTY(QByteArray builderOpMsgId READ builderOpMsgId WRITE
                 setBuilderOpMsgId NOTIFY builderOpMsgIdChanged FINAL)
  Q_PROPERTY(QByteArray conversationId READ conversationId WRITE
                 setConversationId NOTIFY conversationIdChanged FINAL)
  Q_PROPERTY(bool isEmpty READ isEmpty NOTIFY isEmptyChanged FINAL)
  Q_PROPERTY(QString lastAuthor READ lastAuthor NOTIFY lastAuthorChanged FINAL)
  Q_PROPERTY(QString lastBody READ lastBody NOTIFY lastBodyChanged FINAL)
  Q_PROPERTY(QVariant lastStatus READ lastStatus NOTIFY lastStatusChanged FINAL)
  Q_PROPERTY(QVariant lastTime READ lastTime NOTIFY lastTimeChanged FINAL)
  Q_PROPERTY(bool searchActive READ searchActive WRITE setSearchActive NOTIFY
                 searchActiveChanged FINAL)
  Q_PROPERTY(
      quint64 searchIndex READ searchIndex NOTIFY searchIndexChanged FINAL)
  Q_PROPERTY(quint64 searchNumMatches READ searchNumMatches NOTIFY
                 searchNumMatchesChanged FINAL)
  Q_PROPERTY(QString searchPattern READ searchPattern WRITE setSearchPattern
                 NOTIFY searchPatternChanged FINAL)
  Q_PROPERTY(bool searchRegex READ searchRegex WRITE setSearchRegex NOTIFY
                 searchRegexChanged FINAL)
  explicit Messages(bool owned, QObject *parent);

public:
  explicit Messages(QObject *parent = nullptr);
  ~Messages() override;
  const MessageBuilder *builder() const;
  MessageBuilder *builder();
  QByteArray builderOpMsgId() const;
  void setBuilderOpMsgId(const QByteArray &v);
  QByteArray conversationId() const;
  void setConversationId(const QByteArray &v);
  bool isEmpty() const;
  QString lastAuthor() const;
  QString lastBody() const;
  QVariant lastStatus() const;
  QVariant lastTime() const;
  bool searchActive() const;
  void setSearchActive(bool v);
  quint64 searchIndex() const;
  quint64 searchNumMatches() const;
  QString searchPattern() const;
  void setSearchPattern(const QString &v);
  bool searchRegex() const;
  void setSearchRegex(bool v);
  Q_INVOKABLE bool clearConversationHistory();
  Q_INVOKABLE void clearSearch();
  Q_INVOKABLE bool deleteMessage(quint64 row_index);
  Q_INVOKABLE quint64 indexById(const QByteArray &msg_id) const;
  Q_INVOKABLE qint64 nextSearchMatch();
  Q_INVOKABLE qint64 prevSearchMatch();
  Q_INVOKABLE void setSearchHint(float scrollbar_position,
                                 float scrollbar_height);
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  Q_INVOKABLE QString author(int row) const;
  Q_INVOKABLE QString body(int row) const;
  Q_INVOKABLE QVariant dataSaved(int row) const;
  Q_INVOKABLE QVariant expirationTime(int row) const;
  Q_INVOKABLE QVariant hasAttachments(int row) const;
  Q_INVOKABLE QVariant insertionTime(int row) const;
  Q_INVOKABLE QVariant isHead(int row) const;
  Q_INVOKABLE QVariant isTail(int row) const;
  Q_INVOKABLE QVariant matchStatus(int row) const;
  Q_INVOKABLE QByteArray msgId(int row) const;
  Q_INVOKABLE QString opAuthor(int row) const;
  Q_INVOKABLE QString opBody(int row) const;
  Q_INVOKABLE QVariant opHasAttachments(int row) const;
  Q_INVOKABLE QByteArray opMsgId(int row) const;
  Q_INVOKABLE QVariant opTime(int row) const;
  Q_INVOKABLE QVariant receiptStatus(int row) const;
  Q_INVOKABLE QVariant replyType(int row) const;
  Q_INVOKABLE QVariant serverTime(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void builderChanged();
  void builderOpMsgIdChanged();
  void conversationIdChanged();
  void isEmptyChanged();
  void lastAuthorChanged();
  void lastBodyChanged();
  void lastStatusChanged();
  void lastTimeChanged();
  void searchActiveChanged();
  void searchIndexChanged();
  void searchNumMatchesChanged();
  void searchPatternChanged();
  void searchRegexChanged();
};

class Users : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(
      QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
  Q_PROPERTY(bool filterRegex READ filterRegex WRITE setFilterRegex NOTIFY
                 filterRegexChanged FINAL)
  explicit Users(bool owned, QObject *parent);

public:
  explicit Users(QObject *parent = nullptr);
  ~Users() override;
  QString filter() const;
  void setFilter(const QString &v);
  bool filterRegex() const;
  void setFilterRegex(bool v);
  Q_INVOKABLE QByteArray add(const QString &id);
  Q_INVOKABLE void clearFilter();
  Q_INVOKABLE quint32 colorById(const QString &id) const;
  Q_INVOKABLE QString nameById(const QString &id) const;
  Q_INVOKABLE QString profilePictureById(const QString &id) const;
  Q_INVOKABLE bool toggleFilterRegex();
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  bool setData(const QModelIndex &index, const QVariant &value,
               int role = Qt::EditRole) override;
  Q_INVOKABLE quint32 color(int row) const;
  Q_INVOKABLE bool setColor(int row, quint32 value);
  Q_INVOKABLE bool matched(int row) const;
  Q_INVOKABLE QString name(int row) const;
  Q_INVOKABLE bool setName(int row, const QString &value);
  Q_INVOKABLE QByteArray pairwiseConversationId(int row) const;
  Q_INVOKABLE QString profilePicture(int row) const;
  Q_INVOKABLE bool setProfilePicture(int row, const QString &value);
  Q_INVOKABLE quint8 status(int row) const;
  Q_INVOKABLE bool setStatus(int row, quint8 value);
  Q_INVOKABLE QString userId(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void filterChanged();
  void filterRegexChanged();
};

class UsersSearch : public QAbstractItemModel {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  Q_PROPERTY(
      QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
  explicit UsersSearch(bool owned, QObject *parent);

public:
  explicit UsersSearch(QObject *parent = nullptr);
  ~UsersSearch() override;
  QString filter() const;
  void setFilter(const QString &v);
  Q_INVOKABLE void clearFilter();
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = QModelIndex()) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  bool canFetchMore(const QModelIndex &parent) const override;
  void fetchMore(const QModelIndex &parent) override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
  int role(const char *name) const;
  QHash<int, QByteArray> roleNames() const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  bool setHeaderData(int section, Qt::Orientation orientation,
                     const QVariant &value, int role = Qt::EditRole) override;
  Q_INVOKABLE bool
  insertRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;
  Q_INVOKABLE bool
  removeRows(int row, int count,
             const QModelIndex &parent = QModelIndex()) override;

  bool setData(const QModelIndex &index, const QVariant &value,
               int role = Qt::EditRole) override;
  Q_INVOKABLE QVariant color(int row) const;
  Q_INVOKABLE bool matched(int row) const;
  Q_INVOKABLE QString name(int row) const;
  Q_INVOKABLE QString profilePicture(int row) const;
  Q_INVOKABLE bool selected(int row) const;
  Q_INVOKABLE bool setSelected(int row, bool value);
  Q_INVOKABLE QString userId(int row) const;

Q_SIGNALS:
  // new data is ready to be made available to the model with fetchMore()
  void newDataReady(const QModelIndex &parent) const;

private:
  QHash<QPair<int, Qt::ItemDataRole>, QVariant> m_headerData;
  void initHeaderData();
  void updatePersistentIndexes();
Q_SIGNALS:
  void filterChanged();
};

class Utils : public QObject {
  Q_OBJECT
  friend class Herald;
  friend class Messages;

public:
  class Private;

private:
  Private *m_d;
  bool m_ownsPrivate;
  explicit Utils(bool owned, QObject *parent);

public:
  explicit Utils(QObject *parent = nullptr);
  ~Utils() override;
  Q_INVOKABLE bool compareByteArray(const QByteArray &bs1,
                                    const QByteArray &bs2) const;
  Q_INVOKABLE bool isValidRandId(const QByteArray &bs) const;
Q_SIGNALS:
};
#endif // BINDINGS_H
