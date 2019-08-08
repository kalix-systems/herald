#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace ffi {

/// Thin wrapper around sqlite3 database connection.
struct Database;

extern "C" {

/// Creates empty contacts table in database.
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the
/// insertion failed.
int contact_insert(Database *db);

/// Creates empty contacts table in database.
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the table could not
/// be created.
int contacts_create_table(Database *db);

/// Closes connections to canonical sqlite3 database.
void database_close(Database *db);

/// Opens connection to canonical sqlite3 database.
Database *database_open();

} // extern "C"

} // namespace ffi
