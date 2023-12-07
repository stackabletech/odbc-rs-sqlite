# ODBC Driver (for SQLite) written in Rust

This repository contains a _VERY VERY_ experimental rough first draft of an ODBC driver written in Rust.
It is supposed to - at some point - talk to SQLite (and it can actually already list tables under certain conditions).

But it is not the goal of this driver to actually implement a SQLite driver.
We picked SQLite because it requires no external service and is easy to test against.

The main idea and goal of this repository is to see whether writing an ODBC driver in Rust is even feasible.
So far, I'd say: **Yes**

But it will require a lot of work.
The APIs for ODBC are poorly documented and complicated and require a lot of FFI work.

It aims for ODBC 3.8 support.
ODBC 2.x is _NOT_ supported.
                                        
## Resources

These resources helped me while implementing what I have so far:

- [ODBC API/Documentation](https://learn.microsoft.com/en-us/sql/odbc/reference/syntax/odbc-api-reference?view=sql-server-ver16)
  - This is the _bible_ as far as I'm concerned, this has the most details you'll ever get, still not easy to read
  - Also look at the other chapters
  - Don't look at the section about writing drivers :) It contains no useful information
- [Header files](https://github.com/microsoft/ODBC-Specification/blob/master/Windows/inc/sql.h)
  - This repository contains header files for ODBC, which are very useful
  - It is for the unreleased ODBC 4 standard but mostly valid for the older ones as well
- [odbc-sys](https://github.com/pacman82/odbc-sys)
  - This is a `-sys` crate written by [Markus Klein](https://github.com/pacman82) without which I probably wouldn't even have started this project
  - It contains _a lot_ of the necessary definitions to work with ODBC from Rust
  - This crate is geared towards _applications_ wanting to access ODBC resources, it is not originally meant for drivers, so we might have to create a new one for that purpose
  - Thank you!
                  
## Building

```bash
cargo build
```

Should be enough to get you going.
You should end up with a `target/debug/libodbc_driver_rs.so` file.

## Using/Testing

I have _only_ tested this with [unixODBC](https://www.unixodbc.org/) on Linux.
I have not tested on Windows or Mac.
If anyone does: I'd like to hear about it!
   
Create or edit a `~/.odbcinst.ini` file.

```
[odbcrs_sqlite]
Driver = <path to your libodbc_driver_rs.so>
```

`~/.odbc.ini`
```
[test_connection]
Driver = odbcrs_sqlite
Database = <path to your sqlite database>
```
   
Now you should be able to use `isql` to test it:

```
isql -3 test_connection -v
```

(`-3` instructs it to use ODBC 3, without it ODBC 2 is used, which should also work as the Driver Manager should translate calls but...)

The Driver currently spews unconfigurable logs to the console, so you will see all kinds of things on your console.

Once `isql` has started the only command currently doing anything is `help` which lists the tables in the SQLite database.
Also note: The program will log an error if your DB does not exist, but it will NOT (yet) fail as it should!

## Future / Help

This is not an official Stackable product, and we have no immediate plans to develop this any further.
It is mostly a hobby project and a proof-of-concept.

We would, however, be very interested in seeing this continued and collaborate on it.

The idea was to implement this using SQLite to prove it works and then extract all the boilerplate in reusable macros, so it'd be easier to implement drivers going forward.
