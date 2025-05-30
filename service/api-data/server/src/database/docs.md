used strictly for data access (no business logic & services)

# resources: 
- Postgresql to Rust type conversion: https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html#types

# TODO: 
- Handling Spatial Types (GEOGRAPHY, VECTOR):
   The `geom` column in `listing.Location` uses the `extension.geography` type (specifically `Point, 4326`). The `vector` type is also an extension.
   `tokio-postgres` does not have built-in support for these. You will need to:
   a) Choose a Rust crate for representing these spatial types (e.g., `geo-types` for Point).
   b) Manually implement the `tokio_postgres::types::FromSql` and `tokio_postgres::types::ToSql` traits for the chosen Rust spatial type, handling the conversion to/from the database's binary representation of these types. This can be complex. Alternatively, you might fetch these as `String` or `Vec<u8>` and parse them using a spatial library if direct binary mapping is too difficult.
   // You will also need to implement FromSql and ToSql for these types if using tokio-postgres
   // You might need a crate for spatial types, e.g., `geo-types`
   // use geo_types::Point; // Example placeholder


# notes: 
- derive macros (decorators) for structs and enums map the exact name - case sensitive - of the Postgres and graphql equivalent definitions (struct, enum, fields); https://docs.rs/postgres-types/latest/postgres_types/

- PostGIS extension + Rust postgis crate: provide support for geographic binary format support; or string-based conversion using WKT/WKB crates functions. The EWKB extended binary format of WKB. WKT is textual format. PostGIS EWKB format with SRID 4326 geographic coordinate system standard
   - geo-types support convertion with postgres-types to Postgresql built-in geometric types (POINT, PATH, BOX). e.g. geo_types::Point<f64> ↔ PostgreSQL POINT
