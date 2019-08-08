#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace ffi {

/// Thin wrapper around sqlite3 database connection.
struct Database;

/// A constant buffer, templated over the `Item` type.
template<typename Item>
struct ConstBuffer {
  const Item *data;
  uintptr_t len;
};

extern "C" {

/// Frees a ConstBuffer.
void const_buffer_string_free(const ConstBuffer<const char*> *buf);

/// Returns number of items in a `ConstBuffer`
/// Returns -1 on failure.
int const_buffer_string_len(const ConstBuffer<const char*> *buf);

/// Insert default entries into contacts table in database.
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the
/// insertion failed.
int contact_insert(Database *db);

/// Creates empty contacts table in database.
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the table could not
/// be created.
int contacts_create_table(Database *db);

/// Gets a buffer of contact strings.
/// Returns null pointer on failure.
const ConstBuffer<const char*> *contacts_get(Database *db);

/// Closes connections to canonical sqlite3 database.
void database_close(Database *db);

/// Opens connection to canonical sqlite3 database.
/// Returns a null pointer on failure.
Database *database_open();

} // extern "C"

} // namespace ffi
