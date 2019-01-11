#![feature(prelude_import)]
#![no_std]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_mysql;
extern crate serde;

mod routes {

    pub mod forum_page {
        use crate::tables::Category;
        use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};
        pub struct ForumPage {}
        impl ForumPage {
            pub fn build(conn: &MysqlConnection, id: i32) -> Template {
                Template::render("no", ())
            }
        }
    }
    pub mod get {
        use crate::routes::{ForumPage, HomePage, ThreadPage};
        use crate::DBConn;
        use rocket_contrib::templates::Template;
        pub fn homepage(conn: DBConn) -> Template {
            HomePage::build(&conn.0)
        }
        #[doc = r" Rocket code generated wrapping route function."]
        pub fn rocket_route_fn_homepage<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DBConn =
                match <DBConn as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder = homepage(__rocket_param_conn);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        #[doc = r" Rocket code generated wrapping URI macro."]
        macro_rules! rocket_uri_macro_homepage(( $ ( $ token : tt ) * ) => {
                                               {
                                               extern crate std ; extern crate
                                               rocket ; rocket ::
                                               rocket_internal_uri ! (
                                               "/" , (  ) , $ ( $ token ) * )
                                               } });
        #[doc = r" Rocket code generated static route info."]
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_homepage: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "homepage",
                method: ::rocket::http::Method::Get,
                path: "/",
                handler: rocket_route_fn_homepage,
                format: None,
                rank: None,
            };
        pub fn forumpage(conn: DBConn, id: usize) -> Template {
            ForumPage::build(&conn.0, id as i32)
        }
        #[doc = r" Rocket code generated wrapping route function."]
        pub fn rocket_route_fn_forumpage<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DBConn =
                match <DBConn as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_id: usize = match __req.raw_segment_str(1usize) {
                Some(__s) => match <usize as ::rocket::request::FromParam>::from_param(__s) {
                    Ok(__v) => __v,
                    Err(__error) => {
                        return {
                            ::rocket::logger::warn_(&::fmt::format(
                                ::std::fmt::Arguments::new_v1_formatted(
                                    &["Failed to parse \'", "\': "],
                                    &match (&"__rocket_param_id", &__error) {
                                        (arg0, arg1) => [
                                            ::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            ),
                                            ::std::fmt::ArgumentV1::new(
                                                arg1,
                                                ::std::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                    &[
                                        ::std::fmt::rt::v1::Argument {
                                            position: ::std::fmt::rt::v1::Position::At(0usize),
                                            format: ::std::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::std::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::std::fmt::rt::v1::Count::Implied,
                                                width: ::std::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::std::fmt::rt::v1::Argument {
                                            position: ::std::fmt::rt::v1::Position::At(1usize),
                                            format: ::std::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::std::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::std::fmt::rt::v1::Count::Implied,
                                                width: ::std::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                ),
                            ));
                            ::rocket::Outcome::Forward(__data)
                        }
                    }
                },
                None => {
                    return {
                        ::rocket::logger::error(
                            "Internal invariant error: expected dynamic parameter not found.",
                        );
                        ::rocket::logger::error(
                            "Please report this error to the Rocket issue tracker.",
                        );
                        ::rocket::Outcome::Forward(__data)
                    }
                }
            };
            let ___responder = forumpage(__rocket_param_conn, __rocket_param_id);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        #[doc = r" Rocket code generated wrapping URI macro."]
        macro_rules! rocket_uri_macro_forumpage(( $ ( $ token : tt ) * ) => {
                                                {
                                                extern crate std ; extern
                                                crate rocket ; rocket ::
                                                rocket_internal_uri ! (
                                                "/forum/<id>" , ( id : usize )
                                                , $ ( $ token ) * ) } });
        #[doc = r" Rocket code generated static route info."]
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_forumpage: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "forumpage",
                method: ::rocket::http::Method::Get,
                path: "/forum/<id>",
                handler: rocket_route_fn_forumpage,
                format: None,
                rank: None,
            };
        pub fn threadpage(conn: DBConn, id: usize) -> Template {
            ThreadPage::build(&conn.0, id as i32)
        }
        #[doc = r" Rocket code generated wrapping route function."]
        pub fn rocket_route_fn_threadpage<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DBConn =
                match <DBConn as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_id: usize = match __req.raw_segment_str(1usize) {
                Some(__s) => match <usize as ::rocket::request::FromParam>::from_param(__s) {
                    Ok(__v) => __v,
                    Err(__error) => {
                        return {
                            ::rocket::logger::warn_(&::fmt::format(
                                ::std::fmt::Arguments::new_v1_formatted(
                                    &["Failed to parse \'", "\': "],
                                    &match (&"__rocket_param_id", &__error) {
                                        (arg0, arg1) => [
                                            ::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            ),
                                            ::std::fmt::ArgumentV1::new(
                                                arg1,
                                                ::std::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                    &[
                                        ::std::fmt::rt::v1::Argument {
                                            position: ::std::fmt::rt::v1::Position::At(0usize),
                                            format: ::std::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::std::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::std::fmt::rt::v1::Count::Implied,
                                                width: ::std::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::std::fmt::rt::v1::Argument {
                                            position: ::std::fmt::rt::v1::Position::At(1usize),
                                            format: ::std::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::std::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::std::fmt::rt::v1::Count::Implied,
                                                width: ::std::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                ),
                            ));
                            ::rocket::Outcome::Forward(__data)
                        }
                    }
                },
                None => {
                    return {
                        ::rocket::logger::error(
                            "Internal invariant error: expected dynamic parameter not found.",
                        );
                        ::rocket::logger::error(
                            "Please report this error to the Rocket issue tracker.",
                        );
                        ::rocket::Outcome::Forward(__data)
                    }
                }
            };
            let ___responder = threadpage(__rocket_param_conn, __rocket_param_id);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        #[doc = r" Rocket code generated wrapping URI macro."]
        macro_rules! rocket_uri_macro_threadpage(( $ ( $ token : tt ) * ) => {
                                                 {
                                                 extern crate std ; extern
                                                 crate rocket ; rocket ::
                                                 rocket_internal_uri ! (
                                                 "/thread/<id>" , ( id : usize
                                                 ) , $ ( $ token ) * ) } });
        #[doc = r" Rocket code generated static route info."]
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_threadpage: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "threadpage",
                method: ::rocket::http::Method::Get,
                path: "/thread/<id>",
                handler: rocket_route_fn_threadpage,
                format: None,
                rank: None,
            };
    }
    pub mod home_page {
        use crate::tables::Category;
        use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};
        pub struct HomePage {}
        impl HomePage {
            pub fn build(conn: &MysqlConnection) -> Template {
                Template::render("no", ())
            }
        }
    }
    pub mod post {
        use crate::{
            tables::{NewComment, NewThread},
            DBConn,
        };
        use rocket::request::Form;
        pub fn comment(conn: DBConn, new_comment: Form<NewComment>) {
            let new_comment = new_comment.into_inner();
            NewComment::create(&conn.0, new_comment);
        }
        #[doc = r" Rocket code generated wrapping route function."]
        pub fn rocket_route_fn_comment<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DBConn =
                match <DBConn as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let __transform =
                <Form<NewComment> as ::rocket::data::FromData>::transform(__req, __data);
            #[allow(unreachable_patterns, unreachable_code)]
            let __outcome =
                match __transform {
                    ::rocket::data::Transform::Owned(::rocket::Outcome::Success(__v))
                    => {
                        ::rocket::data::Transform::Owned(::rocket::Outcome::Success(__v))
                    }
                    ::rocket::data::Transform::Borrowed(::rocket::Outcome::Success(ref __v))
                    => {
                        ::rocket::data::Transform::Borrowed(::rocket::Outcome::Success(::std::borrow::Borrow::borrow(__v)))
                    }
                    ::rocket::data::Transform::Borrowed(__o) =>
                    ::rocket::data::Transform::Borrowed(__o.map(|_|
                                                                    {
                                                                        {
                                                                            {
                                                                                {
                                                                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1_formatted(&["internal error: entered unreachable code: "],
                                                                                                                                                   &match (&"Borrowed(Success(..)) case handled in previous block",)
                                                                                                                                                        {
                                                                                                                                                        (arg0,)
                                                                                                                                                        =>
                                                                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                     ::std::fmt::Display::fmt)],
                                                                                                                                                    },
                                                                                                                                                   &[::std::fmt::rt::v1::Argument{position:
                                                                                                                                                                                      ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                  format:
                                                                                                                                                                                      ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                         ' ',
                                                                                                                                                                                                                     align:
                                                                                                                                                                                                                         ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                     flags:
                                                                                                                                                                                                                         0u32,
                                                                                                                                                                                                                     precision:
                                                                                                                                                                                                                         ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                     width:
                                                                                                                                                                                                                         ::std::fmt::rt::v1::Count::Implied,},}]),
                                                                                                          &("src/routes/post.rs",
                                                                                                            9u32,
                                                                                                            30u32))
                                                                                }
                                                                            }
                                                                        }
                                                                    })),
                    ::rocket::data::Transform::Owned(__o) =>
                    ::rocket::data::Transform::Owned(__o),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_new_comment: Form<NewComment> =
                match <Form<NewComment> as ::rocket::data::FromData>::from_data(__req, __outcome) {
                    ::rocket::Outcome::Success(__d) => __d,
                    ::rocket::Outcome::Forward(__d) => return ::rocket::Outcome::Forward(__d),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder = comment(__rocket_param_conn, __rocket_param_new_comment);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        #[doc = r" Rocket code generated wrapping URI macro."]
        macro_rules! rocket_uri_macro_comment(( $ ( $ token : tt ) * ) => {
                                              {
                                              extern crate std ; extern crate
                                              rocket ; rocket ::
                                              rocket_internal_uri ! (
                                              "/comment" , (  ) , $ ( $ token
                                              ) * ) } });
        #[doc = r" Rocket code generated static route info."]
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_comment: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "comment",
                method: ::rocket::http::Method::Post,
                path: "/comment",
                handler: rocket_route_fn_comment,
                format: None,
                rank: None,
            };
        pub fn thread(conn: DBConn, new_thread: Form<NewThread>) {
            let new_thread = new_thread.into_inner();
            NewThread::create(&conn.0, new_thread);
        }
        #[doc = r" Rocket code generated wrapping route function."]
        pub fn rocket_route_fn_thread<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DBConn =
                match <DBConn as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let __transform =
                <Form<NewThread> as ::rocket::data::FromData>::transform(__req, __data);
            #[allow(unreachable_patterns, unreachable_code)]
            let __outcome =
                match __transform {
                    ::rocket::data::Transform::Owned(::rocket::Outcome::Success(__v))
                    => {
                        ::rocket::data::Transform::Owned(::rocket::Outcome::Success(__v))
                    }
                    ::rocket::data::Transform::Borrowed(::rocket::Outcome::Success(ref __v))
                    => {
                        ::rocket::data::Transform::Borrowed(::rocket::Outcome::Success(::std::borrow::Borrow::borrow(__v)))
                    }
                    ::rocket::data::Transform::Borrowed(__o) =>
                    ::rocket::data::Transform::Borrowed(__o.map(|_|
                                                                    {
                                                                        {
                                                                            {
                                                                                {
                                                                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1_formatted(&["internal error: entered unreachable code: "],
                                                                                                                                                   &match (&"Borrowed(Success(..)) case handled in previous block",)
                                                                                                                                                        {
                                                                                                                                                        (arg0,)
                                                                                                                                                        =>
                                                                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                     ::std::fmt::Display::fmt)],
                                                                                                                                                    },
                                                                                                                                                   &[::std::fmt::rt::v1::Argument{position:
                                                                                                                                                                                      ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                  format:
                                                                                                                                                                                      ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                         ' ',
                                                                                                                                                                                                                     align:
                                                                                                                                                                                                                         ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                     flags:
                                                                                                                                                                                                                         0u32,
                                                                                                                                                                                                                     precision:
                                                                                                                                                                                                                         ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                     width:
                                                                                                                                                                                                                         ::std::fmt::rt::v1::Count::Implied,},}]),
                                                                                                          &("src/routes/post.rs",
                                                                                                            15u32,
                                                                                                            29u32))
                                                                                }
                                                                            }
                                                                        }
                                                                    })),
                    ::rocket::data::Transform::Owned(__o) =>
                    ::rocket::data::Transform::Owned(__o),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_new_thread: Form<NewThread> =
                match <Form<NewThread> as ::rocket::data::FromData>::from_data(__req, __outcome) {
                    ::rocket::Outcome::Success(__d) => __d,
                    ::rocket::Outcome::Forward(__d) => return ::rocket::Outcome::Forward(__d),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder = thread(__rocket_param_conn, __rocket_param_new_thread);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        #[doc = r" Rocket code generated wrapping URI macro."]
        macro_rules! rocket_uri_macro_thread(( $ ( $ token : tt ) * ) => {
                                             {
                                             extern crate std ; extern crate
                                             rocket ; rocket ::
                                             rocket_internal_uri ! (
                                             "/thread" , (  ) , $ ( $ token )
                                             * ) } });
        #[doc = r" Rocket code generated static route info."]
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_thread: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "thread",
                method: ::rocket::http::Method::Post,
                path: "/thread",
                handler: rocket_route_fn_thread,
                format: None,
                rank: None,
            };
    }
    pub mod thread_page {
        use crate::tables::Category;
        use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};
        pub struct ThreadPage {}
        impl ThreadPage {
            pub fn build(conn: &MysqlConnection, id: i32) -> Template {
                Template::render("no", ())
            }
        }
    }
    pub use crate::routes::{forum_page::ForumPage, home_page::HomePage, thread_page::ThreadPage};
}
mod schema {
    pub mod categories {
        #![allow(dead_code)]
        pub use self::columns::*;
        use associations::HasTable;
        use insertable::Insertable;
        use query_builder::nodes::Identifier;
        use query_builder::*;
        use query_source::joins::{Join, JoinOn};
        use query_source::{AppearsInFromClause, Never, Once};
        use sql_types::*;
        use {JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            macro_rules! __static_cond(( categories categories ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( id ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( categories id ) => {
                                       pub use super :: columns :: { id } ;
                                       });
            pub use super::columns::id;
            macro_rules! __static_cond(( categories categories ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( name ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( name ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( name ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( categories name ) => {
                                       pub use super :: columns :: { name } ;
                                       });
            pub use super::columns::name;
            pub use super::table as categories;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, name) = (id, name);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_query_id_for_table {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("categories")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = columns::id;
            type AllColumns = (id, name);
            fn primary_key(&self) -> Self::PrimaryKey {
                columns::id
            }
            fn all_columns() -> Self::AllColumns {
                (id, name)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use backend::Backend;
            use query_builder::{AstPass, QueryFragment, SelectStatement};
            use query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use query_source::{AppearsInFromClause, Never, Once};
            use result::QueryResult;
            use sql_types::*;
            use {AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for id {}
            impl ::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::EqAll<T> for id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_name {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name
                }
            }
            impl ::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for name
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for name {}
            impl ::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::EqAll<T> for name
            where
                T: ::expression::AsExpression<Text>,
                ::dsl::Eq<name, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod comments {
        #![allow(dead_code)]
        pub use self::columns::*;
        use associations::HasTable;
        use insertable::Insertable;
        use query_builder::nodes::Identifier;
        use query_builder::*;
        use query_source::joins::{Join, JoinOn};
        use query_source::{AppearsInFromClause, Never, Once};
        use sql_types::*;
        use {JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            macro_rules! __static_cond(( comments comments ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( id ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( comments id ) => {
                                       pub use super :: columns :: { id } ;
                                       });
            pub use super::columns::id;
            macro_rules! __static_cond(( comments comments ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( thread_id )
                                       ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( thread_id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( thread_id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( comments thread_id ) => {
                                       pub use super :: columns :: { thread_id
                                       } ; });
            pub use super::columns::thread_id;
            macro_rules! __static_cond(( comments comments ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( content ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( content ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( content ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( comments content ) => {
                                       pub use super :: columns :: { content }
                                       ; });
            pub use super::columns::content;
            macro_rules! __static_cond(( comments comments ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( created_at )
                                       ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( created_at ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( created_at ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( comments created_at ) => {
                                       pub use super :: columns :: {
                                       created_at } ; });
            pub use super::columns::created_at;
            pub use super::table as comments;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, thread_id, content, created_at) =
            (id, thread_id, content, created_at);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_query_id_for_table {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Text, Timestamp);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("comments")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = columns::id;
            type AllColumns = (id, thread_id, content, created_at);
            fn primary_key(&self) -> Self::PrimaryKey {
                columns::id
            }
            fn all_columns() -> Self::AllColumns {
                (id, thread_id, content, created_at)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use backend::Backend;
            use query_builder::{AstPass, QueryFragment, SelectStatement};
            use query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use query_source::{AppearsInFromClause, Never, Once};
            use result::QueryResult;
            use sql_types::*;
            use {AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for id {}
            impl ::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::EqAll<T> for id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct thread_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for thread_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        thread_id => {
                            let mut debug_trait_builder = f.debug_tuple("thread_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for thread_id {
                #[inline]
                fn clone(&self) -> thread_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for thread_id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_thread_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for thread_id {
                    type QueryId = thread_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for thread_id {
                #[inline]
                fn default() -> thread_id {
                    thread_id
                }
            }
            impl ::expression::Expression for thread_id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for thread_id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("thread_id")
                }
            }
            impl SelectableExpression<table> for thread_id {}
            impl<QS> AppearsOnTable<QS> for thread_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for thread_id
            where
                thread_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for thread_id
            where
                thread_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for thread_id where
                thread_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for thread_id where
                thread_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for thread_id {}
            impl ::query_source::Column for thread_id {
                type Table = table;
                const NAME: &'static str = "thread_id";
            }
            impl<T> ::EqAll<T> for thread_id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<thread_id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for thread_id
            where
                Rhs: ::expression::AsExpression<
                    <<thread_id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for thread_id
            where
                Rhs: ::expression::AsExpression<
                    <<thread_id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for thread_id
            where
                Rhs: ::expression::AsExpression<
                    <<thread_id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for thread_id
            where
                Rhs: ::expression::AsExpression<
                    <<thread_id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct content;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for content {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        content => {
                            let mut debug_trait_builder = f.debug_tuple("content");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for content {
                #[inline]
                fn clone(&self) -> content {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for content {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_content {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for content {
                    type QueryId = content;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for content {
                #[inline]
                fn default() -> content {
                    content
                }
            }
            impl ::expression::Expression for content {
                type SqlType = Text;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for content
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("content")
                }
            }
            impl SelectableExpression<table> for content {}
            impl<QS> AppearsOnTable<QS> for content where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for content
            where
                content: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for content
            where
                content: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for content where
                content: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for content where
                content: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for content {}
            impl ::query_source::Column for content {
                type Table = table;
                const NAME: &'static str = "content";
            }
            impl<T> ::EqAll<T> for content
            where
                T: ::expression::AsExpression<Text>,
                ::dsl::Eq<content, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_at => {
                            let mut debug_trait_builder = f.debug_tuple("created_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_at {
                #[inline]
                fn clone(&self) -> created_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_at {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_created_at {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_at {
                    type QueryId = created_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_at {
                #[inline]
                fn default() -> created_at {
                    created_at
                }
            }
            impl ::expression::Expression for created_at {
                type SqlType = Timestamp;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for created_at
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_at")
                }
            }
            impl SelectableExpression<table> for created_at {}
            impl<QS> AppearsOnTable<QS> for created_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_at where
                created_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for created_at where
                created_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for created_at {}
            impl ::query_source::Column for created_at {
                type Table = table;
                const NAME: &'static str = "created_at";
            }
            impl<T> ::EqAll<T> for created_at
            where
                T: ::expression::AsExpression<Timestamp>,
                ::dsl::Eq<created_at, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for created_at
            where
                Rhs: ::expression::AsExpression<
                    <<created_at as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for created_at
            where
                Rhs: ::expression::AsExpression<
                    <<created_at as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod forums {
        #![allow(dead_code)]
        pub use self::columns::*;
        use associations::HasTable;
        use insertable::Insertable;
        use query_builder::nodes::Identifier;
        use query_builder::*;
        use query_source::joins::{Join, JoinOn};
        use query_source::{AppearsInFromClause, Never, Once};
        use sql_types::*;
        use {JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            macro_rules! __static_cond(( forums forums ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( id ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( forums id ) => {
                                       pub use super :: columns :: { id } ;
                                       });
            pub use super::columns::id;
            macro_rules! __static_cond(( forums forums ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( name ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( name ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( name ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( forums name ) => {
                                       pub use super :: columns :: { name } ;
                                       });
            pub use super::columns::name;
            macro_rules! __static_cond(( forums forums ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( category_id
                                       ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( category_id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( category_id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( forums category_id ) => {
                                       pub use super :: columns :: {
                                       category_id } ; });
            pub use super::columns::category_id;
            macro_rules! __static_cond(( forums forums ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! (
                                       max_threads_number ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( max_threads_number ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( max_threads_number ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( forums max_threads_number )
                                       => {
                                       pub use super :: columns :: {
                                       max_threads_number } ; });
            pub use super::columns::max_threads_number;
            macro_rules! __static_cond(( forums forums ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! (
                                       cur_threads_number ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( cur_threads_number ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( cur_threads_number ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( forums cur_threads_number )
                                       => {
                                       pub use super :: columns :: {
                                       cur_threads_number } ; });
            pub use super::columns::cur_threads_number;
            pub use super::table as forums;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            name,
            category_id,
            max_threads_number,
            cur_threads_number,
        ) = (
            id,
            name,
            category_id,
            max_threads_number,
            cur_threads_number,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_query_id_for_table {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Text, Integer, Integer, Integer);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("forums")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = columns::id;
            type AllColumns = (
                id,
                name,
                category_id,
                max_threads_number,
                cur_threads_number,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                columns::id
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    name,
                    category_id,
                    max_threads_number,
                    cur_threads_number,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use backend::Backend;
            use query_builder::{AstPass, QueryFragment, SelectStatement};
            use query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use query_source::{AppearsInFromClause, Never, Once};
            use result::QueryResult;
            use sql_types::*;
            use {AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for id {}
            impl ::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::EqAll<T> for id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_name {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name
                }
            }
            impl ::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for name
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for name {}
            impl ::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::EqAll<T> for name
            where
                T: ::expression::AsExpression<Text>,
                ::dsl::Eq<name, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct category_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for category_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        category_id => {
                            let mut debug_trait_builder = f.debug_tuple("category_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for category_id {
                #[inline]
                fn clone(&self) -> category_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for category_id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_category_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for category_id {
                    type QueryId = category_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for category_id {
                #[inline]
                fn default() -> category_id {
                    category_id
                }
            }
            impl ::expression::Expression for category_id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for category_id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("category_id")
                }
            }
            impl SelectableExpression<table> for category_id {}
            impl<QS> AppearsOnTable<QS> for category_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for category_id
            where
                category_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for category_id
            where
                category_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for category_id where
                category_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for category_id where
                category_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for category_id {}
            impl ::query_source::Column for category_id {
                type Table = table;
                const NAME: &'static str = "category_id";
            }
            impl<T> ::EqAll<T> for category_id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<category_id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for category_id
            where
                Rhs: ::expression::AsExpression<
                    <<category_id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for category_id
            where
                Rhs: ::expression::AsExpression<
                    <<category_id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for category_id
            where
                Rhs: ::expression::AsExpression<
                    <<category_id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for category_id
            where
                Rhs: ::expression::AsExpression<
                    <<category_id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct max_threads_number;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for max_threads_number {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        max_threads_number => {
                            let mut debug_trait_builder = f.debug_tuple("max_threads_number");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for max_threads_number {
                #[inline]
                fn clone(&self) -> max_threads_number {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for max_threads_number {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_max_threads_number {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for max_threads_number {
                    type QueryId = max_threads_number;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for max_threads_number {
                #[inline]
                fn default() -> max_threads_number {
                    max_threads_number
                }
            }
            impl ::expression::Expression for max_threads_number {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for max_threads_number
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("max_threads_number")
                }
            }
            impl SelectableExpression<table> for max_threads_number {}
            impl<QS> AppearsOnTable<QS> for max_threads_number where QS: AppearsInFromClause<table, Count = Once>
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for max_threads_number
            where
                max_threads_number: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for max_threads_number
            where
                max_threads_number: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for max_threads_number where
                max_threads_number: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for max_threads_number where
                max_threads_number:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for max_threads_number {}
            impl ::query_source::Column for max_threads_number {
                type Table = table;
                const NAME: &'static str = "max_threads_number";
            }
            impl<T> ::EqAll<T> for max_threads_number
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<max_threads_number, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for max_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<max_threads_number as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for max_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<max_threads_number as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for max_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<max_threads_number as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for max_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<max_threads_number as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct cur_threads_number;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for cur_threads_number {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        cur_threads_number => {
                            let mut debug_trait_builder = f.debug_tuple("cur_threads_number");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for cur_threads_number {
                #[inline]
                fn clone(&self) -> cur_threads_number {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for cur_threads_number {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_cur_threads_number {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for cur_threads_number {
                    type QueryId = cur_threads_number;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for cur_threads_number {
                #[inline]
                fn default() -> cur_threads_number {
                    cur_threads_number
                }
            }
            impl ::expression::Expression for cur_threads_number {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for cur_threads_number
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("cur_threads_number")
                }
            }
            impl SelectableExpression<table> for cur_threads_number {}
            impl<QS> AppearsOnTable<QS> for cur_threads_number where QS: AppearsInFromClause<table, Count = Once>
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for cur_threads_number
            where
                cur_threads_number: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for cur_threads_number
            where
                cur_threads_number: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for cur_threads_number where
                cur_threads_number: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for cur_threads_number where
                cur_threads_number:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for cur_threads_number {}
            impl ::query_source::Column for cur_threads_number {
                type Table = table;
                const NAME: &'static str = "cur_threads_number";
            }
            impl<T> ::EqAll<T> for cur_threads_number
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<cur_threads_number, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for cur_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<cur_threads_number as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for cur_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<cur_threads_number as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for cur_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<cur_threads_number as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for cur_threads_number
            where
                Rhs: ::expression::AsExpression<
                    <<cur_threads_number as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod pictures {
        #![allow(dead_code)]
        pub use self::columns::*;
        use associations::HasTable;
        use insertable::Insertable;
        use query_builder::nodes::Identifier;
        use query_builder::*;
        use query_source::joins::{Join, JoinOn};
        use query_source::{AppearsInFromClause, Never, Once};
        use sql_types::*;
        use {JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            macro_rules! __static_cond(( pictures pictures ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( id ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( pictures id ) => {
                                       pub use super :: columns :: { id } ;
                                       });
            pub use super::columns::id;
            macro_rules! __static_cond(( pictures pictures ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( comment_id )
                                       ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( comment_id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( comment_id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( pictures comment_id ) => {
                                       pub use super :: columns :: {
                                       comment_id } ; });
            pub use super::columns::comment_id;
            macro_rules! __static_cond(( pictures pictures ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! (
                                       path_to_picture ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( path_to_picture ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( path_to_picture ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( pictures path_to_picture )
                                       => {
                                       pub use super :: columns :: {
                                       path_to_picture } ; });
            pub use super::columns::path_to_picture;
            pub use super::table as pictures;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, comment_id, path_to_picture) =
            (id, comment_id, path_to_picture);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_query_id_for_table {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("pictures")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = columns::id;
            type AllColumns = (id, comment_id, path_to_picture);
            fn primary_key(&self) -> Self::PrimaryKey {
                columns::id
            }
            fn all_columns() -> Self::AllColumns {
                (id, comment_id, path_to_picture)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use backend::Backend;
            use query_builder::{AstPass, QueryFragment, SelectStatement};
            use query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use query_source::{AppearsInFromClause, Never, Once};
            use result::QueryResult;
            use sql_types::*;
            use {AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for id {}
            impl ::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::EqAll<T> for id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct comment_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for comment_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        comment_id => {
                            let mut debug_trait_builder = f.debug_tuple("comment_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for comment_id {
                #[inline]
                fn clone(&self) -> comment_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for comment_id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_comment_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for comment_id {
                    type QueryId = comment_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for comment_id {
                #[inline]
                fn default() -> comment_id {
                    comment_id
                }
            }
            impl ::expression::Expression for comment_id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for comment_id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("comment_id")
                }
            }
            impl SelectableExpression<table> for comment_id {}
            impl<QS> AppearsOnTable<QS> for comment_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for comment_id
            where
                comment_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for comment_id
            where
                comment_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for comment_id where
                comment_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for comment_id where
                comment_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for comment_id {}
            impl ::query_source::Column for comment_id {
                type Table = table;
                const NAME: &'static str = "comment_id";
            }
            impl<T> ::EqAll<T> for comment_id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<comment_id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for comment_id
            where
                Rhs: ::expression::AsExpression<
                    <<comment_id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for comment_id
            where
                Rhs: ::expression::AsExpression<
                    <<comment_id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for comment_id
            where
                Rhs: ::expression::AsExpression<
                    <<comment_id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for comment_id
            where
                Rhs: ::expression::AsExpression<
                    <<comment_id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct path_to_picture;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for path_to_picture {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        path_to_picture => {
                            let mut debug_trait_builder = f.debug_tuple("path_to_picture");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for path_to_picture {
                #[inline]
                fn clone(&self) -> path_to_picture {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for path_to_picture {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_path_to_picture {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for path_to_picture {
                    type QueryId = path_to_picture;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for path_to_picture {
                #[inline]
                fn default() -> path_to_picture {
                    path_to_picture
                }
            }
            impl ::expression::Expression for path_to_picture {
                type SqlType = Text;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for path_to_picture
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("path_to_picture")
                }
            }
            impl SelectableExpression<table> for path_to_picture {}
            impl<QS> AppearsOnTable<QS> for path_to_picture where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for path_to_picture
            where
                path_to_picture: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for path_to_picture
            where
                path_to_picture: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for path_to_picture where
                path_to_picture: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for path_to_picture where
                path_to_picture: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for path_to_picture {}
            impl ::query_source::Column for path_to_picture {
                type Table = table;
                const NAME: &'static str = "path_to_picture";
            }
            impl<T> ::EqAll<T> for path_to_picture
            where
                T: ::expression::AsExpression<Text>,
                ::dsl::Eq<path_to_picture, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod threads {
        #![allow(dead_code)]
        pub use self::columns::*;
        use associations::HasTable;
        use insertable::Insertable;
        use query_builder::nodes::Identifier;
        use query_builder::*;
        use query_source::joins::{Join, JoinOn};
        use query_source::{AppearsInFromClause, Never, Once};
        use sql_types::*;
        use {JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            macro_rules! __static_cond(( threads threads ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( id ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( threads id ) => {
                                       pub use super :: columns :: { id } ;
                                       });
            pub use super::columns::id;
            macro_rules! __static_cond(( threads threads ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( forum_id ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( forum_id ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( forum_id ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( threads forum_id ) => {
                                       pub use super :: columns :: { forum_id
                                       } ; });
            pub use super::columns::forum_id;
            macro_rules! __static_cond(( threads threads ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( title ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( title ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( title ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( threads title ) => {
                                       pub use super :: columns :: { title } ;
                                       });
            pub use super::columns::title;
            macro_rules! __static_cond(( threads threads ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( position ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( position ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( position ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( threads position ) => {
                                       pub use super :: columns :: { position
                                       } ; });
            pub use super::columns::position;
            macro_rules! __static_cond(( threads threads ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( created_at )
                                       ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( created_at ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( created_at ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( threads created_at ) => {
                                       pub use super :: columns :: {
                                       created_at } ; });
            pub use super::columns::created_at;
            macro_rules! __static_cond(( threads threads ) => {
                                       compile_error ! (
                                       concat ! (
                                       "Column `" , stringify ! ( bump ) ,
                                       "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \""
                                       , stringify ! ( bump ) ,
                                       "\"]` to reference the table's `" ,
                                       stringify ! ( bump ) ,
                                       "` column. \n \
                            Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`\n"
                                       ) ) ; } ; ( threads bump ) => {
                                       pub use super :: columns :: { bump } ;
                                       });
            pub use super::columns::bump;
            pub use super::table as threads;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, forum_id, title, position, created_at, bump) =
            (id, forum_id, title, position, created_at, bump);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_query_id_for_table {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Text, Integer, Timestamp, Integer);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("threads")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = columns::id;
            type AllColumns = (id, forum_id, title, position, created_at, bump);
            fn primary_key(&self) -> Self::PrimaryKey {
                columns::id
            }
            fn all_columns() -> Self::AllColumns {
                (id, forum_id, title, position, created_at, bump)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use backend::Backend;
            use query_builder::{AstPass, QueryFragment, SelectStatement};
            use query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use query_source::{AppearsInFromClause, Never, Once};
            use result::QueryResult;
            use sql_types::*;
            use {AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for id {}
            impl ::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::EqAll<T> for id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::expression::AsExpression<
                    <<id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct forum_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for forum_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        forum_id => {
                            let mut debug_trait_builder = f.debug_tuple("forum_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for forum_id {
                #[inline]
                fn clone(&self) -> forum_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for forum_id {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_forum_id {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for forum_id {
                    type QueryId = forum_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for forum_id {
                #[inline]
                fn default() -> forum_id {
                    forum_id
                }
            }
            impl ::expression::Expression for forum_id {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for forum_id
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("forum_id")
                }
            }
            impl SelectableExpression<table> for forum_id {}
            impl<QS> AppearsOnTable<QS> for forum_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for forum_id
            where
                forum_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for forum_id
            where
                forum_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for forum_id where
                forum_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for forum_id where
                forum_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for forum_id {}
            impl ::query_source::Column for forum_id {
                type Table = table;
                const NAME: &'static str = "forum_id";
            }
            impl<T> ::EqAll<T> for forum_id
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<forum_id, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for forum_id
            where
                Rhs: ::expression::AsExpression<
                    <<forum_id as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for forum_id
            where
                Rhs: ::expression::AsExpression<
                    <<forum_id as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for forum_id
            where
                Rhs: ::expression::AsExpression<
                    <<forum_id as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for forum_id
            where
                Rhs: ::expression::AsExpression<
                    <<forum_id as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct title;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for title {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        title => {
                            let mut debug_trait_builder = f.debug_tuple("title");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for title {
                #[inline]
                fn clone(&self) -> title {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for title {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_title {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for title {
                    type QueryId = title;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for title {
                #[inline]
                fn default() -> title {
                    title
                }
            }
            impl ::expression::Expression for title {
                type SqlType = Text;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for title
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("title")
                }
            }
            impl SelectableExpression<table> for title {}
            impl<QS> AppearsOnTable<QS> for title where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for title
            where
                title: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for title
            where
                title: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for title where
                title: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for title where
                title: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for title {}
            impl ::query_source::Column for title {
                type Table = table;
                const NAME: &'static str = "title";
            }
            impl<T> ::EqAll<T> for title
            where
                T: ::expression::AsExpression<Text>,
                ::dsl::Eq<title, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct position;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for position {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        position => {
                            let mut debug_trait_builder = f.debug_tuple("position");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for position {
                #[inline]
                fn clone(&self) -> position {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for position {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_position {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for position {
                    type QueryId = position;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for position {
                #[inline]
                fn default() -> position {
                    position
                }
            }
            impl ::expression::Expression for position {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for position
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("position")
                }
            }
            impl SelectableExpression<table> for position {}
            impl<QS> AppearsOnTable<QS> for position where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for position
            where
                position: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for position
            where
                position: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for position where
                position: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for position where
                position: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for position {}
            impl ::query_source::Column for position {
                type Table = table;
                const NAME: &'static str = "position";
            }
            impl<T> ::EqAll<T> for position
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<position, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for position
            where
                Rhs: ::expression::AsExpression<
                    <<position as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for position
            where
                Rhs: ::expression::AsExpression<
                    <<position as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for position
            where
                Rhs: ::expression::AsExpression<
                    <<position as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for position
            where
                Rhs: ::expression::AsExpression<
                    <<position as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct created_at;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for created_at {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        created_at => {
                            let mut debug_trait_builder = f.debug_tuple("created_at");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for created_at {
                #[inline]
                fn clone(&self) -> created_at {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for created_at {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_created_at {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for created_at {
                    type QueryId = created_at;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for created_at {
                #[inline]
                fn default() -> created_at {
                    created_at
                }
            }
            impl ::expression::Expression for created_at {
                type SqlType = Timestamp;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for created_at
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("created_at")
                }
            }
            impl SelectableExpression<table> for created_at {}
            impl<QS> AppearsOnTable<QS> for created_at where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for created_at
            where
                created_at: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for created_at where
                created_at: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for created_at where
                created_at: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for created_at {}
            impl ::query_source::Column for created_at {
                type Table = table;
                const NAME: &'static str = "created_at";
            }
            impl<T> ::EqAll<T> for created_at
            where
                T: ::expression::AsExpression<Timestamp>,
                ::dsl::Eq<created_at, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for created_at
            where
                Rhs: ::expression::AsExpression<
                    <<created_at as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for created_at
            where
                Rhs: ::expression::AsExpression<
                    <<created_at as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct bump;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for bump {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        bump => {
                            let mut debug_trait_builder = f.debug_tuple("bump");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for bump {
                #[inline]
                fn clone(&self) -> bump {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for bump {}
            #[allow(non_snake_case, unused_extern_crates)]
            mod _impl_query_id_for_bump {
                extern crate std;
                mod diesel {
                    pub use *;
                }
                use self::diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for bump {
                    type QueryId = bump;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for bump {
                #[inline]
                fn default() -> bump {
                    bump
                }
            }
            impl ::expression::Expression for bump {
                type SqlType = Integer;
            }
            impl<DB> ::query_builder::QueryFragment<DB> for bump
            where
                DB: ::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::query_builder::AstPass<DB>,
                ) -> ::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("bump")
                }
            }
            impl SelectableExpression<table> for bump {}
            impl<QS> AppearsOnTable<QS> for bump where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for bump
            where
                bump: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for bump
            where
                bump: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {}
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for bump where
                bump: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {}
            impl<From> SelectableExpression<SelectStatement<From>> for bump where
                bump: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {}
            impl ::expression::NonAggregate for bump {}
            impl ::query_source::Column for bump {
                type Table = table;
                const NAME: &'static str = "bump";
            }
            impl<T> ::EqAll<T> for bump
            where
                T: ::expression::AsExpression<Integer>,
                ::dsl::Eq<bump, T>: ::Expression<SqlType = ::sql_types::Bool>,
            {
                type Output = ::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for bump
            where
                Rhs: ::expression::AsExpression<
                    <<bump as ::Expression>::SqlType as ::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for bump
            where
                Rhs: ::expression::AsExpression<
                    <<bump as ::Expression>::SqlType as ::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for bump
            where
                Rhs: ::expression::AsExpression<
                    <<bump as ::Expression>::SqlType as ::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for bump
            where
                Rhs: ::expression::AsExpression<
                    <<bump as ::Expression>::SqlType as ::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
        }
    }
    impl ::JoinTo<threads::table> for comments::table {
        type FromClause = threads::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<comments::thread_id>,
            ::expression::nullable::Nullable<<threads::table as ::query_source::Table>::PrimaryKey>,
        >;
        fn join_target(rhs: threads::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                comments::thread_id.nullable().eq(
                    <threads::table as ::query_source::Table>::primary_key(&threads::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::JoinTo<comments::table> for threads::table {
        type FromClause = comments::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<comments::thread_id>,
            ::expression::nullable::Nullable<<threads::table as ::query_source::Table>::PrimaryKey>,
        >;
        fn join_target(rhs: comments::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                comments::thread_id.nullable().eq(
                    <threads::table as ::query_source::Table>::primary_key(&threads::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::JoinTo<categories::table> for forums::table {
        type FromClause = categories::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<forums::category_id>,
            ::expression::nullable::Nullable<
                <categories::table as ::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: categories::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                forums::category_id
                    .nullable()
                    .eq(<categories::table as ::query_source::Table>::primary_key(
                        &categories::table,
                    ).nullable()),
            )
        }
    }
    impl ::JoinTo<forums::table> for categories::table {
        type FromClause = forums::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<forums::category_id>,
            ::expression::nullable::Nullable<
                <categories::table as ::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: forums::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                forums::category_id
                    .nullable()
                    .eq(<categories::table as ::query_source::Table>::primary_key(
                        &categories::table,
                    ).nullable()),
            )
        }
    }
    impl ::JoinTo<comments::table> for pictures::table {
        type FromClause = comments::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<pictures::comment_id>,
            ::expression::nullable::Nullable<
                <comments::table as ::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: comments::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                pictures::comment_id.nullable().eq(
                    <comments::table as ::query_source::Table>::primary_key(&comments::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::JoinTo<pictures::table> for comments::table {
        type FromClause = pictures::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<pictures::comment_id>,
            ::expression::nullable::Nullable<
                <comments::table as ::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: pictures::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                pictures::comment_id.nullable().eq(
                    <comments::table as ::query_source::Table>::primary_key(&comments::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::JoinTo<forums::table> for threads::table {
        type FromClause = forums::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<threads::forum_id>,
            ::expression::nullable::Nullable<<forums::table as ::query_source::Table>::PrimaryKey>,
        >;
        fn join_target(rhs: forums::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                threads::forum_id.nullable().eq(
                    <forums::table as ::query_source::Table>::primary_key(&forums::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::JoinTo<threads::table> for forums::table {
        type FromClause = threads::table;
        type OnClause = ::dsl::Eq<
            ::expression::nullable::Nullable<threads::forum_id>,
            ::expression::nullable::Nullable<<forums::table as ::query_source::Table>::PrimaryKey>,
        >;
        fn join_target(rhs: threads::table) -> (Self::FromClause, Self::OnClause) {
            use {ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                threads::forum_id.nullable().eq(
                    <forums::table as ::query_source::Table>::primary_key(&forums::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::query_source::AppearsInFromClause<categories::table> for comments::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<comments::table> for categories::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<categories::table> for forums::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<forums::table> for categories::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<categories::table> for pictures::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<pictures::table> for categories::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<categories::table> for threads::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<threads::table> for categories::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<comments::table> for forums::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<forums::table> for comments::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<comments::table> for pictures::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<pictures::table> for comments::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<comments::table> for threads::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<threads::table> for comments::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<forums::table> for pictures::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<pictures::table> for forums::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<forums::table> for threads::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<threads::table> for forums::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<pictures::table> for threads::table {
        type Count = ::query_source::Never;
    }
    impl ::query_source::AppearsInFromClause<threads::table> for pictures::table {
        type Count = ::query_source::Never;
    }
}
mod tables {
    mod category {
        use diesel::prelude::*;
        use rocket_contrib::databases::diesel::MysqlConnection;
        pub struct Category {
            id: i32,
            name: String,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_queryable_for_category {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Category
            where
                (i32, String): Queryable<__ST, __DB>,
            {
                type Row = <(i32, String) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (i32, String) = Queryable::build(row);
                    Self {
                        id: (row.0),
                        name: (row.1),
                    }
                }
            }
        }
        impl Category {
            pub fn all(conn: &MysqlConnection) -> Vec<Category> {
                use crate::schema::categories::dsl::*;
                categories.load::<Category>(conn).expect("Oh no..")
            }
            pub fn id(&self) -> i32 {
                self.id
            }
            pub fn name(self) -> String {
                self.name
            }
        }
    }
    mod comment {
        use chrono::NaiveDateTime;
        use crate::{
            schema::{comments, threads},
            tables::thread::Thread,
        };
        use diesel::{prelude::*, result::Error};
        use rocket_contrib::databases::diesel::MysqlConnection;
        use std::cmp::Ordering;
        pub struct Comment {
            id: i32,
            thread_id: i32,
            content: String,
            created_at: NaiveDateTime,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_queryable_for_comment {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Comment
            where
                (i32, i32, String, NaiveDateTime): Queryable<__ST, __DB>,
            {
                type Row = <(i32, i32, String, NaiveDateTime) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (i32, i32, String, NaiveDateTime) = Queryable::build(row);
                    Self {
                        id: (row.0),
                        thread_id: (row.1),
                        content: (row.2),
                        created_at: (row.3),
                    }
                }
            }
        }
        impl Comment {
            fn by_thread(conn: &MysqlConnection, thread_id: i32) -> Vec<Comment> {
                use crate::schema::comments::dsl::*;
                comments
                    .filter(thread_id.eq(thread_id))
                    .then_order_by(created_at.desc())
                    .load::<Comment>(conn)
                    .expect("Oh no..")
            }
        }
        #[table_name = "comments"]
        pub struct NewComment {
            thread_id: i32,
            content: String,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_insertable_for_newcomment {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::insertable::Insertable;
            use self::diesel::prelude::*;
            use self::diesel::query_builder::UndecoratedInsertRecord;
            impl<'insert> Insertable<comments::table> for NewComment {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<comments::thread_id, i32>>,
                    std::option::Option<diesel::dsl::Eq<comments::content, String>>,
                ) as Insertable<comments::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(comments::thread_id.eq(self.thread_id)),
                        std::option::Option::Some(comments::content.eq(self.content)),
                    )
                        .values()
                }
            }
            impl<'insert> Insertable<comments::table> for &'insert NewComment {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<comments::thread_id, &'insert i32>>,
                    std::option::Option<diesel::dsl::Eq<comments::content, &'insert String>>,
                ) as Insertable<comments::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(comments::thread_id.eq(&self.thread_id)),
                        std::option::Option::Some(comments::content.eq(&self.content)),
                    )
                        .values()
                }
            }
            impl<'insert> UndecoratedInsertRecord<comments::table> for NewComment {}
        }
        impl<'__f> ::rocket::request::FromForm<'__f> for NewComment {
            type Error = ::rocket::request::FormParseError<'__f>;
            fn from_form(
                __items: &mut ::rocket::request::FormItems<'__f>,
                __strict: bool,
            ) -> ::std::result::Result<Self, Self::Error> {
                let mut thread_id = None;
                let mut content = None;
                for (__k, __v) in __items.map(|item| item.key_value()) {
                    match __k.as_str() {
                        "thread_id" => {
                            thread_id = Some(
                                <i32 as ::rocket::request::FromFormValue>::from_form_value(__v)
                                    .map_err(|_| {
                                        ::rocket::request::FormParseError::BadValue(__k, __v)
                                    })?,
                            );
                        }
                        "content" => {
                            content = Some(
                                <String as ::rocket::request::FromFormValue>::from_form_value(__v)
                                    .map_err(|_| {
                                        ::rocket::request::FormParseError::BadValue(__k, __v)
                                    })?,
                            );
                        }
                        _ if __strict && __k != "_method" => {
                            return Err(::rocket::request::FormParseError::Unknown(__k, __v));
                        }
                        _ => {}
                    }
                }
                Ok(Self {
                    thread_id: thread_id
                        .or_else(<i32 as ::rocket::request::FromFormValue>::default)
                        .ok_or_else(|| {
                            ::rocket::request::FormParseError::Missing("thread_id".into())
                        })?,
                    content: content
                        .or_else(<String as ::rocket::request::FromFormValue>::default)
                        .ok_or_else(|| {
                            ::rocket::request::FormParseError::Missing("content".into())
                        })?,
                })
            }
        }
        impl NewComment {
            pub fn create(conn: &MysqlConnection, new_comment: NewComment) {
                let create_comment = diesel::insert_into(comments::table).values(&new_comment);
                conn.transaction::<_, Error, _>(|| {
                    let thread = Thread::by_id(conn, new_comment.thread_id)?
                        .pop()
                        .expect("On ho..");
                    match thread.position().cmp(&1) {
                        Ordering::Equal => {}
                        _ => {
                            let _ = Thread::lower_until(conn, thread.position())?;
                            diesel::update(threads::table)
                                .filter(threads::id.eq(new_comment.thread_id))
                                .set(threads::position.eq(1))
                                .execute(conn)?;
                        }
                    };
                    create_comment.execute(conn)?;
                    Ok(())
                }).unwrap();
            }
        }
    }
    mod forum {
        use diesel::prelude::*;
        use rocket_contrib::databases::diesel::MysqlConnection;
        pub struct Forum {
            id: i32,
            name: String,
            category_id: i32,
            max_threads_number: i32,
            cur_threads_number: i32,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_queryable_for_forum {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Forum
            where
                (i32, String, i32, i32, i32): Queryable<__ST, __DB>,
            {
                type Row = <(i32, String, i32, i32, i32) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (i32, String, i32, i32, i32) = Queryable::build(row);
                    Self {
                        id: (row.0),
                        name: (row.1),
                        category_id: (row.2),
                        max_threads_number: (row.3),
                        cur_threads_number: (row.4),
                    }
                }
            }
        }
        impl Forum {
            pub fn id(&self) -> i32 {
                self.id
            }
            pub fn max_threads_number(&self) -> i32 {
                self.max_threads_number
            }
            pub fn cur_threads_number(&self) -> i32 {
                self.max_threads_number
            }
        }
        impl Forum {
            pub fn by_category(conn: &MysqlConnection, category_id: i32) -> Vec<Forum> {
                use crate::schema::forums::dsl::*;
                forums
                    .filter(category_id.eq(category_id))
                    .load::<Forum>(conn)
                    .expect("Oh no..")
            }
            pub fn by_id(conn: &MysqlConnection, id: i32) -> QueryResult<Vec<Forum>> {
                use crate::schema::forums::dsl::*;
                forums.filter(id.eq(id)).limit(1).load::<Forum>(conn)
            }
        }
    }
    mod picture {
        use crate::schema::pictures;
        use diesel::prelude::*;
        use rocket_contrib::databases::diesel::MysqlConnection;
        pub struct Picture {
            id: i32,
            comment_id: i32,
            path_to_picture: String,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_queryable_for_picture {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Picture
            where
                (i32, i32, String): Queryable<__ST, __DB>,
            {
                type Row = <(i32, i32, String) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (i32, i32, String) = Queryable::build(row);
                    Self {
                        id: (row.0),
                        comment_id: (row.1),
                        path_to_picture: (row.2),
                    }
                }
            }
        }
        impl Picture {
            pub fn by_comment(conn: &MysqlConnection, comment_id: i32) -> Vec<Picture> {
                use crate::schema::pictures::dsl::*;
                pictures
                    .filter(comment_id.eq(comment_id))
                    .load::<Picture>(conn)
                    .expect("Oh no..")
            }
        }
        #[table_name = "pictures"]
        pub struct NewPicture {
            comment_id: i32,
            path_to_picture: String,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_insertable_for_newpicture {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::insertable::Insertable;
            use self::diesel::prelude::*;
            use self::diesel::query_builder::UndecoratedInsertRecord;
            impl<'insert> Insertable<pictures::table> for NewPicture {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<pictures::comment_id, i32>>,
                    std::option::Option<diesel::dsl::Eq<pictures::path_to_picture, String>>,
                ) as Insertable<pictures::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(pictures::comment_id.eq(self.comment_id)),
                        std::option::Option::Some(
                            pictures::path_to_picture.eq(self.path_to_picture),
                        ),
                    )
                        .values()
                }
            }
            impl<'insert> Insertable<pictures::table> for &'insert NewPicture {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<pictures::comment_id, &'insert i32>>,
                    std::option::Option<
                        diesel::dsl::Eq<pictures::path_to_picture, &'insert String>,
                    >,
                ) as Insertable<pictures::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(pictures::comment_id.eq(&self.comment_id)),
                        std::option::Option::Some(
                            pictures::path_to_picture.eq(&self.path_to_picture),
                        ),
                    )
                        .values()
                }
            }
            impl<'insert> UndecoratedInsertRecord<pictures::table> for NewPicture {}
        }
        impl NewPicture {
            pub fn create(
                conn: &MysqlConnection,
                path_to_picture: String,
                comment_id: i32,
            ) -> QueryResult<usize> {
                let picture = NewPicture {
                    comment_id,
                    path_to_picture,
                };
                diesel::insert_into(pictures::table)
                    .values(&picture)
                    .execute(conn)
            }
        }
    }
    mod thread {
        use chrono::NaiveDateTime;
        use crate::{
            schema::{forums, threads},
            tables::forum::Forum,
        };
        use diesel::{prelude::*, result::Error};
        use rocket_contrib::databases::diesel::MysqlConnection;
        use std::cmp::Ordering;
        pub struct Thread {
            id: i32,
            forum_id: i32,
            title: String,
            position: i32,
            created_at: NaiveDateTime,
            bump: i32,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_queryable_for_thread {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Thread
            where
                (i32, i32, String, i32, NaiveDateTime, i32): Queryable<__ST, __DB>,
            {
                type Row =
                    <(i32, i32, String, i32, NaiveDateTime, i32) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (i32, i32, String, i32, NaiveDateTime, i32) = Queryable::build(row);
                    Self {
                        id: (row.0),
                        forum_id: (row.1),
                        title: (row.2),
                        position: (row.3),
                        created_at: (row.4),
                        bump: (row.5),
                    }
                }
            }
        }
        impl Thread {
            pub fn id(&self) -> i32 {
                self.id
            }
            pub fn position(&self) -> i32 {
                self.position
            }
            pub fn bump(&self) -> i32 {
                self.bump
            }
        }
        impl Thread {
            pub fn by_forum(conn: &MysqlConnection, forum_id: i32) -> Vec<Thread> {
                use crate::schema::threads::dsl::*;
                threads
                    .filter(forum_id.eq(forum_id))
                    .load::<Thread>(conn)
                    .expect("Oh no..")
            }
            pub fn by_id(conn: &MysqlConnection, thread_id: i32) -> QueryResult<Vec<Thread>> {
                use crate::schema::threads::dsl::*;
                threads
                    .filter(id.eq(thread_id))
                    .limit(1)
                    .load::<Thread>(conn)
            }
            pub fn lower_all(conn: &MysqlConnection) -> QueryResult<usize> {
                diesel::update(threads::table)
                    .set(threads::position.eq(threads::position + 1))
                    .execute(conn)
            }
            pub fn lower_until(conn: &MysqlConnection, position: i32) -> QueryResult<usize> {
                diesel::update(threads::table)
                    .set(threads::position.eq(threads::position + 1))
                    .filter(threads::position.lt(position))
                    .execute(conn)
            }
        }
        #[table_name = "threads"]
        pub struct NewThread {
            forum_id: i32,
            title: String,
        }
        #[allow(non_snake_case, unused_extern_crates)]
        mod _impl_insertable_for_newthread {
            extern crate std;
            mod diesel {
                pub use *;
            }
            use self::diesel::insertable::Insertable;
            use self::diesel::prelude::*;
            use self::diesel::query_builder::UndecoratedInsertRecord;
            impl<'insert> Insertable<threads::table> for NewThread {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<threads::forum_id, i32>>,
                    std::option::Option<diesel::dsl::Eq<threads::title, String>>,
                ) as Insertable<threads::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(threads::forum_id.eq(self.forum_id)),
                        std::option::Option::Some(threads::title.eq(self.title)),
                    )
                        .values()
                }
            }
            impl<'insert> Insertable<threads::table> for &'insert NewThread {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<threads::forum_id, &'insert i32>>,
                    std::option::Option<diesel::dsl::Eq<threads::title, &'insert String>>,
                ) as Insertable<threads::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(threads::forum_id.eq(&self.forum_id)),
                        std::option::Option::Some(threads::title.eq(&self.title)),
                    )
                        .values()
                }
            }
            impl<'insert> UndecoratedInsertRecord<threads::table> for NewThread {}
        }
        impl<'__f> ::rocket::request::FromForm<'__f> for NewThread {
            type Error = ::rocket::request::FormParseError<'__f>;
            fn from_form(
                __items: &mut ::rocket::request::FormItems<'__f>,
                __strict: bool,
            ) -> ::std::result::Result<Self, Self::Error> {
                let mut forum_id = None;
                let mut title = None;
                for (__k, __v) in __items.map(|item| item.key_value()) {
                    match __k.as_str() {
                        "forum_id" => {
                            forum_id = Some(
                                <i32 as ::rocket::request::FromFormValue>::from_form_value(__v)
                                    .map_err(|_| {
                                        ::rocket::request::FormParseError::BadValue(__k, __v)
                                    })?,
                            );
                        }
                        "title" => {
                            title = Some(
                                <String as ::rocket::request::FromFormValue>::from_form_value(__v)
                                    .map_err(|_| {
                                        ::rocket::request::FormParseError::BadValue(__k, __v)
                                    })?,
                            );
                        }
                        _ if __strict && __k != "_method" => {
                            return Err(::rocket::request::FormParseError::Unknown(__k, __v));
                        }
                        _ => {}
                    }
                }
                Ok(Self {
                    forum_id: forum_id
                        .or_else(<i32 as ::rocket::request::FromFormValue>::default)
                        .ok_or_else(|| {
                            ::rocket::request::FormParseError::Missing("forum_id".into())
                        })?,
                    title: title
                        .or_else(<String as ::rocket::request::FromFormValue>::default)
                        .ok_or_else(|| {
                            ::rocket::request::FormParseError::Missing("title".into())
                        })?,
                })
            }
        }
        impl NewThread {
            pub fn create(conn: &MysqlConnection, new_thread: NewThread) {
                let create_thread = diesel::insert_into(threads::table).values(&new_thread);
                conn.transaction::<_, Error, _>(|| {
                    Thread::lower_all(conn)?;
                    create_thread.execute(conn)?;
                    let forum = Forum::by_id(conn, new_thread.forum_id)?
                        .pop()
                        .expect("No such forum");
                    match forum.cur_threads_number().cmp(&forum.max_threads_number()) {
                        Ordering::Equal => diesel::delete(threads::table)
                            .filter(threads::position.gt(forum.max_threads_number()))
                            .execute(conn),
                        _ => diesel::update(forums::table)
                            .set(forums::cur_threads_number.eq(forums::cur_threads_number + 1))
                            .execute(conn),
                    }?;
                    Ok(())
                }).expect("Oh no..");
            }
        }
    }
    pub use crate::tables::{
        category::Category,
        comment::{Comment, NewComment},
        forum::Forum,
        picture::{NewPicture, Picture},
        thread::{NewThread, Thread},
    };
}
use crate::routes::{get, post};
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};
pub const HOST: &str = "localhost:7999";
#[doc = r" The request guard type."]
pub struct DBConn(
    pub  ::rocket_contrib::databases::r2d2::PooledConnection<
        <MysqlConnection as ::rocket_contrib::databases::Poolable>::Manager,
    >,
);
#[doc = r" The pool type."]
pub struct DBConnPool(
    ::rocket_contrib::databases::r2d2::Pool<
        <MysqlConnection as ::rocket_contrib::databases::Poolable>::Manager,
    >,
);
impl DBConn {
    #[doc = r" Returns a fairing that initializes the associated database"]
    #[doc = r" connection pool."]
    pub fn fairing() -> impl ::rocket::fairing::Fairing {
        use rocket_contrib::databases::Poolable;
        ::rocket::fairing::AdHoc::on_attach("\'mysql\' Database Pool", |rocket| {
            let pool = ::rocket_contrib::databases::database_config("mysql", rocket.config())
                .map(MysqlConnection::pool);
            match pool {
                Ok(Ok(p)) => Ok(rocket.manage(DBConnPool(p))),
                Err(config_error) => {
                    ::rocket::logger::error(&::fmt::format(
                        ::std::fmt::Arguments::new_v1_formatted(
                            &["Database configuration failure: \'", "\'"],
                            &match (&"mysql",) {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                                }
                            },
                            &[::std::fmt::rt::v1::Argument {
                                position: ::std::fmt::rt::v1::Position::At(0usize),
                                format: ::std::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::std::fmt::rt::v1::Count::Implied,
                                    width: ::std::fmt::rt::v1::Count::Implied,
                                },
                            }],
                        ),
                    ));
                    ::rocket::logger::error_(&::fmt::format(
                        ::std::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &match (&config_error,) {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                                }
                            },
                            &[::std::fmt::rt::v1::Argument {
                                position: ::std::fmt::rt::v1::Position::At(0usize),
                                format: ::std::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::std::fmt::rt::v1::Count::Implied,
                                    width: ::std::fmt::rt::v1::Count::Implied,
                                },
                            }],
                        ),
                    ));
                    Err(rocket)
                }
                Ok(Err(pool_error)) => {
                    ::rocket::logger::error(&::fmt::format(
                        ::std::fmt::Arguments::new_v1_formatted(
                            &["Failed to initialize pool for \'", "\'"],
                            &match (&"mysql",) {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                                }
                            },
                            &[::std::fmt::rt::v1::Argument {
                                position: ::std::fmt::rt::v1::Position::At(0usize),
                                format: ::std::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::std::fmt::rt::v1::Count::Implied,
                                    width: ::std::fmt::rt::v1::Count::Implied,
                                },
                            }],
                        ),
                    ));
                    ::rocket::logger::error_(&::fmt::format(
                        ::std::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &match (&pool_error,) {
                                (arg0,) => {
                                    [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)]
                                }
                            },
                            &[::std::fmt::rt::v1::Argument {
                                position: ::std::fmt::rt::v1::Position::At(0usize),
                                format: ::std::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::std::fmt::rt::v1::Count::Implied,
                                    width: ::std::fmt::rt::v1::Count::Implied,
                                },
                            }],
                        ),
                    ));
                    Err(rocket)
                }
            }
        })
    }
    #[doc = r" Retrieves a connection of type `Self` from the `rocket`"]
    #[doc = r" instance. Returns `Some` as long as `Self::fairing()` has been"]
    #[doc = r" attached and there is at least one connection in the pool."]
    pub fn get_one(rocket: &::rocket::Rocket) -> Option<Self> {
        rocket
            .state::<DBConnPool>()
            .and_then(|pool| pool.0.get().ok())
            .map(DBConn)
    }
}
impl ::std::ops::Deref for DBConn {
    type Target = MysqlConnection;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for DBConn {
    type Error = ();
    fn from_request(
        request: &'a ::rocket::request::Request<'r>,
    ) -> ::rocket::request::Outcome<Self, ()> {
        use rocket::{http::Status, Outcome};
        let pool = request.guard::<::rocket::State<DBConnPool>>()?;
        match pool.0.get() {
            Ok(conn) => Outcome::Success(DBConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
fn main() {
    rocket::ignite()
        .mount("/", {
            let __vector: Vec<::rocket::Route> = <[_]>::into_vec(box [
                ::rocket::Route::from(&get::static_rocket_route_info_for_homepage),
                ::rocket::Route::from(&get::static_rocket_route_info_for_forumpage),
                ::rocket::Route::from(&get::static_rocket_route_info_for_threadpage),
                ::rocket::Route::from(&post::static_rocket_route_info_for_comment),
                ::rocket::Route::from(&post::static_rocket_route_info_for_thread),
            ]);
            __vector
        }).attach(DBConn::fairing())
        .attach(Template::fairing())
        .launch();
}
