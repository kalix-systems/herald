/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Config;
class Contacts;
class Messages;
class NetworkHandle;

class Config : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint32 color READ color WRITE setColor NOTIFY colorChanged FINAL)
    Q_PROPERTY(quint32 colorscheme READ colorscheme WRITE setColorscheme NOTIFY colorschemeChanged FINAL)
    Q_PROPERTY(QString configId READ configId WRITE setConfigId NOTIFY configIdChanged FINAL)
    Q_PROPERTY(bool init READ init NOTIFY initChanged FINAL)
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged FINAL)
    Q_PROPERTY(QString profilePicture READ profilePicture WRITE setProfilePicture NOTIFY profilePictureChanged FINAL)
    explicit Config(bool owned, QObject *parent);
public:
    explicit Config(QObject *parent = nullptr);
    ~Config();
    quint32 color() const;
    void setColor(quint32 v);
    quint32 colorscheme() const;
    void setColorscheme(quint32 v);
    QString configId() const;
    void setConfigId(const QString& v);
    bool init() const;
    QString name() const;
    void setName(const QString& v);
    QString profilePicture() const;
    void setProfilePicture(const QString& v);
    Q_INVOKABLE bool exists() const;
Q_SIGNALS:
    void colorChanged();
    void colorschemeChanged();
    void configIdChanged();
    void initChanged();
    void nameChanged();
    void profilePictureChanged();
};

class Contacts : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
    Q_PROPERTY(bool filterRegex READ filterRegex WRITE setFilterRegex NOTIFY filterRegexChanged FINAL)
    explicit Contacts(bool owned, QObject *parent);
public:
    explicit Contacts(QObject *parent = nullptr);
    ~Contacts();
    QString filter() const;
    void setFilter(const QString& v);
    bool filterRegex() const;
    void setFilterRegex(bool v);
    Q_INVOKABLE bool add(const QString& id);
    Q_INVOKABLE bool toggleFilterRegex();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool archiveStatus(int row) const;
    Q_INVOKABLE bool setArchiveStatus(int row, bool value);
    Q_INVOKABLE quint32 color(int row) const;
    Q_INVOKABLE bool setColor(int row, quint32 value);
    Q_INVOKABLE QString contactId(int row) const;
    Q_INVOKABLE bool matched(int row) const;
    Q_INVOKABLE bool setMatched(int row, bool value);
    Q_INVOKABLE QString name(int row) const;
    Q_INVOKABLE bool setName(int row, const QString& value);
    Q_INVOKABLE QString profilePicture(int row) const;
    Q_INVOKABLE bool setProfilePicture(int row, const QString& value);

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void filterChanged();
    void filterRegexChanged();
};

class Messages : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QByteArray conversationId READ conversationId WRITE setConversationId NOTIFY conversationIdChanged FINAL)
    explicit Messages(bool owned, QObject *parent);
public:
    explicit Messages(QObject *parent = nullptr);
    ~Messages();
    QByteArray conversationId() const;
    void setConversationId(const QByteArray& v);
    Q_INVOKABLE void clearConversationView();
    Q_INVOKABLE bool deleteConversationById(const QByteArray& conversation_id);
    Q_INVOKABLE bool deleteMessage(quint64 row_index);
    Q_INVOKABLE bool delete_conversation();
    Q_INVOKABLE bool insertMessage(const QString& body);
    Q_INVOKABLE bool reply(const QString& body, const QByteArray& op);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QString author(int row) const;
    Q_INVOKABLE QString body(int row) const;
    Q_INVOKABLE qint64 epochTimestampMs(int row) const;
    Q_INVOKABLE QByteArray messageId(int row) const;
    Q_INVOKABLE QByteArray op(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void conversationIdChanged();
};

class NetworkHandle : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(bool connectionPending READ connectionPending NOTIFY connectionPendingChanged FINAL)
    Q_PROPERTY(bool connectionUp READ connectionUp NOTIFY connectionUpChanged FINAL)
    Q_PROPERTY(bool newMessage READ newMessage NOTIFY newMessageChanged FINAL)
    explicit NetworkHandle(bool owned, QObject *parent);
public:
    explicit NetworkHandle(QObject *parent = nullptr);
    ~NetworkHandle();
    bool connectionPending() const;
    bool connectionUp() const;
    bool newMessage() const;
    Q_INVOKABLE bool registerDevice();
    Q_INVOKABLE bool requestMetaData(const QString& of);
    Q_INVOKABLE bool sendMessage(const QString& message_body, const QString& to);
Q_SIGNALS:
    void connectionPendingChanged();
    void connectionUpChanged();
    void newMessageChanged();
};
#endif // BINDINGS_H
