#[cfg(test)]
mod tests {
    use heck::ToUpperCamelCase;
    use quick_xml::{events::Event, name::QName, Reader};

    struct Column {
        name: String,
        type_: String,
        length: i32,
        nullable: bool,
        extension: bool,
        revision: bool,
        primary_key: bool,
    }

    struct Table {
        name: String,
        columns: Vec<Column>,
    }

    /// Generates all files for repositories.
    #[test]
    fn generate_reps() {
        let mut tables: Vec<Table> = Vec::new();
        read_tables(&mut tables);
        // Generates files.
        let t = tables
            .iter()
            .filter(|a| {
                !a.name.starts_with("HP_")
                    && !a.name.starts_with("MO_")
                    && !a.name.starts_with("SO_xxx")
                    && !a.name.starts_with("VM_")
                    && a.name == "TB_Eintrag_Ort"
            })
            .collect::<Vec<_>>();
        if self::mach_nichts() == 0 {
            let sb = create_reps(&t);
            println!("{}", sb);
        } else if self::mach_nichts() == 1 {
            let sb = create_undo_entry(&t);
            println!("{}", sb);
        } else if self::mach_nichts() == 1 {
            std::fs::write(
                "/home/wolfgang/rust/rabp/rep/src/schema.rs",
                create_schema(&t),
            )
            .unwrap();
            //} else if self::mach_nichts() == 1 {
            std::fs::write(
                "/home/wolfgang/rust/rabp/rep/src/models.rs",
                create_models(&t),
            )
            .unwrap();
        }
    }

    /// Generates repositories.
    fn create_reps(tables: &Vec<&Table>) -> String {
        let mut sb = String::new();
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "use crate::{{
    base::{{
        errors::Result,
        reps::{{mach_angelegt, mach_geaendert}},
        undo::UndoEntry,
    }},
    ServiceData, ServiceError,
}};
use chrono::{{NaiveDate, NaiveDateTime}};
use diesel::{{prelude::*, SqliteConnection}};
use rep::{{models::{}, schema::{}}};
",
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                )
                .as_str(),
            );

            // Undo
            sb.push_str(
                format!(
                    "
/// Undoes dataset.
pub fn undo(con: &mut SqliteConnection, data: &mut ServiceData, or: &String, ac: &String) -> Result<()> {{
    let oo = UndoEntry::from_str::<{}>(or)?;
    let oa = UndoEntry::from_str::<{}>(ac)?;
    if let (Some(o), Some(_a)) = (&oo, &oa) {{
        // Update
        update(con, data, o)?;
    }} else if let Some(a) = &oa {{
        // Insert
        delete(con, data, a)?;
    }} else if let Some(o) = &oo {{
        // Delete
        insert(con, data, o)?;
    }}
    Ok(())
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Redo
            sb.push_str(
                format!(
                    "
/// Redoes dataset.
pub fn redo(con: &mut SqliteConnection, data: &mut ServiceData, or: &String, ac: &String) -> Result<()> {{
    let oo = UndoEntry::from_str::<{}>(or)?;
    let oa = UndoEntry::from_str::<{}>(ac)?;
    if let (Some(_o), Some(a)) = (&oo, &oa) {{
        // Update
        update(con, data, a)?;
    }} else if let Some(a) = &oa {{
        // Insert
        insert(con, data, a)?;
    }} else if let Some(o) = &oo {{
        // Delete
        delete(con, data, o)?;
    }}
    Ok(())
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Save0
            let parms = t
                .columns
                .iter()
                .map(|a| {
                    format!(
                        "
    {}_: &{}",
                        a.name.to_lowercase(),
                        get_rust_type(a)
                    )
                })
                .collect::<Vec<String>>()
                .join(",");
            let mut cf = 0;
            let filter = t
                .columns
                .iter()
                .filter(|a| a.primary_key)
                .map(|a| {
                    cf += 1;
                    format!(
                        "
            {}{}::{}.eq({}_{}){}",
                        self::iif(cf > 1, ".and(", ""),
                        t.name.to_uppercase(),
                        a.name.to_lowercase(),
                        a.name.to_lowercase(),
                        get_rust_type_clone(a),
                        self::iif(cf > 1, ")", ""),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            let init = t
                .columns
                .iter()
                .map(|a| {
                    if a.revision && !a.name.to_lowercase().starts_with("replikation_uid") {
                        return format!(
                            "
        {}: None,",
                            a.name.to_lowercase()
                        );
                    }
                    let cl = get_rust_type_clone(a);
                    format!(
                        "
        {}: {}{}_{},",
                        a.name.to_lowercase(),
                        self::iif(cl.len() <= 0, "*", ""),
                        a.name.to_lowercase(),
                        cl,
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Saves dataset with all values.
#[allow(dead_code)]
pub fn save0(
    con: &mut SqliteConnection, data: &mut ServiceData,{}
) -> Result<{}> {{
    let op = {}::table
        .filter({},
        )
        .first::<{}>(con)
        .optional()?;
    let mut p = {} {{{}
    }};
    if let Some(pu) = op {{
        if p != pu {{
        p.angelegt_von = pu.angelegt_von;
        p.angelegt_am = pu.angelegt_am;
        p.geaendert_von = pu.geaendert_von;
        p.geaendert_am = pu.geaendert_am;
        if p.angelegt_von.is_none() || !angelegt_von_.is_none() {{
                mach_angelegt(&mut p, data, angelegt_von_, angelegt_am_);
            }}
            mach_geaendert(&mut p, data, geaendert_von_, geaendert_am_);
            update(con, data, &p)?;
        }}
    }} else {{
        mach_angelegt(&mut p, data, angelegt_von_, angelegt_am_);
        if !geaendert_von_.is_none() {{
            mach_geaendert(&mut p, data, geaendert_von_, geaendert_am_);
        }}
        insert(con, data, &p)?;
    }}
    return Ok(p);
}}
",
                    parms,
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter,
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    init,
                )
                .as_str(),
            );

            // Save
            let parms_rv = t
                .columns
                .iter()
                .filter(|a| !a.revision)
                .map(|a| {
                    format!(
                        "
    {}_: &{}",
                        a.name.to_lowercase(),
                        get_rust_type(a)
                    )
                })
                .collect::<Vec<String>>()
                .join(",");
            let parms2 = t
                .columns
                .iter()
                .map(|a| {
                    if a.revision {
                        return format!(
                            "
        &None,"
                        );
                    }
                    format!(
                        "
        {}_,",
                        a.name.to_lowercase(),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Saves dataset without revision columns.
#[allow(dead_code)]
pub fn save(
    con: &mut SqliteConnection, data: &mut ServiceData,{}
) -> Result<{}> {{
    save0(
        con, data,{}
    )
}}
",
                    parms_rv,
                    t.name.to_upper_camel_case(),
                    parms2,
                )
                .as_str(),
            );

            // Get
            let parms_pk = t
                .columns
                .iter()
                .filter(|a| a.primary_key)
                .map(|a| {
                    format!(
                        "
    {}_: &{}",
                        a.name.to_lowercase(),
                        get_rust_type(a)
                    )
                })
                .collect::<Vec<String>>()
                .join(",");
            sb.push_str(
                format!(
                    "
/// Gets dataset by primary key.
#[allow(dead_code)]
pub fn get(
    con: &mut SqliteConnection,{}
) -> Result<Option<{}>> {{
    let p = {}::table
        .filter({},
        )
        .first::<{}>(con)
        .optional()?;
    Ok(p)
}}
",
                    parms_pk,
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Get2
            cf = 0;
            let filter2 = t
                .columns
                .iter()
                .filter(|a| a.primary_key)
                .map(|a| {
                    cf += 1;
                    format!(
                        "
            {}{}::{}.eq(b.{}{}){}",
                        self::iif(cf > 1, ".and(", ""),
                        t.name.to_uppercase(),
                        a.name.to_lowercase(),
                        a.name.to_lowercase(),
                        get_rust_type_clone(a),
                        self::iif(cf > 1, ")", ""),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Gets dataset by primary key.
pub fn get2(con: &mut SqliteConnection, b: &{}) -> Result<Option<{}>> {{
    let p = {}::table
        .filter({},
        )
        .first::<{}>(con)
        .optional()?;
    Ok(p)
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter2,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Get_List
            let with_client = t
                .columns
                .iter()
                .any(|a| a.name.to_lowercase() == "mandant_nr");
            sb.push_str(
                format!(
                    "
/// Gets list.
#[allow(dead_code)]
pub fn get_list(con: &mut SqliteConnection{}) -> Result<Vec<{}>> {{
    let list = {}::table{}
        .load::<{}>(con)?;
    Ok(list)
}}
",
                    self::iif(with_client, ", mandant_nr_: i32", ""),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    self::iif(
                        with_client,
                        format!(
                            "
        .filter({}::mandant_nr.eq(mandant_nr_))",
                            t.name.to_uppercase()
                        )
                        .as_str(),
                        ""
                    ),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Insert
            sb.push_str(
                format!(
                    "
/// Inserts dataset.
pub fn insert<'a>(con: &mut SqliteConnection, data: &mut ServiceData, b: &'a {}) -> Result<&'a {}> {{
    let rows = diesel::insert_into({}::table).values(b).execute(con)?;
    if rows <= 0 {{
        return Err(ServiceError::NotFound);
    }}
    data.ul.add(&UndoEntry::{}(None, Some(b)));
    Ok(b)
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    t.name.to_lowercase(),
                )
                .as_str(),
            );

            // Update
            let set = t
                .columns
                .iter()
                .filter(|a| !a.primary_key)
                .map(|a| {
                    cf += 1;
                    format!(
                        "
        {}::{}.eq(b.{}{}),",
                        t.name.to_uppercase(),
                        a.name.to_lowercase(),
                        a.name.to_lowercase(),
                        get_rust_type_as_ref(a),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Updates dataset.
pub fn update<'a>(con: &mut SqliteConnection, data: &mut ServiceData, b: &'a {}) -> Result<&'a {}> {{
    let oo = get2(con, b)?;
    let rows = diesel::update(
        {}::table.filter({},
        ),
    )
    .set(({}
    ))
    .execute(con)?;
    if rows <= 0 || oo.is_none() {{
        return Err(ServiceError::NotFound);
    }}
    if let Some(o) = oo {{
        data.ul.add(&UndoEntry::{}(Some(&o), Some(b)));
    }}
    Ok(b)
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter2,
                    set,
                    t.name.to_lowercase(),
                )
                .as_str(),
            );

            // Delete
            sb.push_str(
                format!(
                    "
/// Delets dataset.
pub fn delete(con: &mut SqliteConnection, data: &mut ServiceData, b: &{}) -> Result<()> {{
    let oo = get2(con, b)?;
    let rows = diesel::delete(
        {}::table.filter({},
        ),
    )
    .execute(con)?;
    if rows <= 0 || oo.is_none() {{
        return Err(ServiceError::NotFound);
    }}
    if let Some(o) = oo {{
        data.ul.add(&UndoEntry::{}(Some(&o), None));
    }}
    Ok(())
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter2,
                    t.name.to_lowercase(),
                )
                .as_str(),
            );
        }
        sb
    }

    /// Generate UndoEntry.
    fn create_undo_entry(tables: &Vec<&Table>) -> String {
        let mut sb = String::new();
        let j = tables
            .iter()
            .map(|a| a.name.to_upper_camel_case())
            .collect::<Vec<String>>()
            .join(", ");
        sb.push_str(
            format!(
                "use super::super::reps;
use super::errors::Result;
use crate::base::service::ServiceData;
use rep::models::{{{}}};
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug)]
pub enum UndoEntry {{
",
                j.as_str()
            )
            .as_str(),
        );
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    {} {{ original: String, actual: String }},
",
                    t.name.to_upper_camel_case()
                )
                .as_str(),
            );
        }
        sb.push_str(
            "}

impl UndoEntry {
",
        );
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    pub fn {}(original: Option<&{}>, actual: Option<&{}>) -> Self {{
        UndoEntry::{} {{
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }}
    }}
",
                    t.name.to_lowercase(),
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
        }
        sb.push_str(
            "}
",
        );

        // Undo
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    UndoEntry::{} {{ original, actual }} => {{
        reps::{}::undo(con, data, original, actual)?;
    }}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_lowercase(),
                )
                .as_str(),
            );
        }

        // Redo
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    UndoEntry::{} {{ original, actual }} => {{
        reps::{}::redo(con, data, original, actual)?;
    }}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_lowercase(),
                )
                .as_str(),
            );
        }
        sb
    }

    /// Generates file schema.rs.
    fn create_schema(tables: &Vec<&Table>) -> String {
        let mut sb = r#"use diesel::{allow_tables_to_appear_in_same_query, table};
"#
        .to_string();
        for t in tables.iter() {
            // Table
            let mut pk = String::new();
            for (i, c) in t.columns.iter().enumerate() {
                if c.primary_key {
                    if i > 0 {
                        pk.push_str(", ");
                    }
                    pk.push_str(c.name.to_lowercase().as_str());
                }
            }
            sb.push_str(
                format!(
                    r#"
table! {{
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    {} ({}) {{
"#,
                    t.name.to_uppercase(),
                    pk,
                )
                .as_str(),
            );
            for c in t.columns.iter() {
                sb.push_str(
                    format!(
                        r#"        {} -> {},
"#,
                        c.name.to_lowercase(),
                        get_diesel_type(c),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"    }
}
"#,
            );
        }
        sb.push_str(
            r#"
allow_tables_to_appear_in_same_query!("#,
        );
        for t in tables.iter() {
            sb.push_str(
                format!(
                    r#"
{},"#,
                    t.name.to_uppercase()
                )
                .as_str(),
            );
        }
        sb.push_str(
            r#"
);
"#,
        );
        sb
    }

    /// Generates file models.rs.
    fn create_models(tables: &Vec<&Table>) -> String {
        let j = tables
            .iter()
            .map(|a| a.name.to_uppercase())
            .collect::<Vec<String>>()
            .join(", ");
        let mut sb = format!(
            r#"use crate::{{
    revision::Revision,
    schema::{{{}}},
}};
use chrono::{{NaiveDate, NaiveDateTime}};
use diesel;
use serde::{{Deserialize, Serialize}};
"#,
            j,
        );
        for t in tables.iter() {
            // Model
            sb.push_str(
                format!(
                    r#"
#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = {})]
#[allow(non_snake_case)]
pub struct {} {{
"#,
                    t.name.to_uppercase(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
            for c in t.columns.iter() {
                sb.push_str(
                    format!(
                        r#"    pub {}: {},
"#,
                        c.name.to_lowercase(),
                        get_rust_type(c),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"}
"#,
            );

            // Clone
            sb.push_str(
                format!(
                    r#"
impl Clone for {} {{
    fn clone(&self) -> Self {{
        Self {{
"#,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
            for c in t.columns.iter() {
                sb.push_str(
                    format!(
                        r#"            {}: self.{}{},
"#,
                        c.name.to_lowercase(),
                        c.name.to_lowercase(),
                        get_rust_type_clone(c),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"        }
    }
}
"#,
            );

            // PartialEq, comparison without Revision
            sb.push_str(
                format!(
                    r#"
impl PartialEq for {} {{
    fn eq(&self, other: &Self) -> bool {{
"#,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
            for (i, c) in t.columns.iter().filter(|a| !a.revision).enumerate() {
                sb.push_str(
                    format!(
                        r#"        {}self.{} == other.{}
"#,
                        self::iif(i == 0, "", "    && "),
                        c.name.to_lowercase(),
                        c.name.to_lowercase(),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"    }
}
"#,
            );

            if !t.name.starts_with("SO_") {
                // Revision
                sb.push_str(
                    format!(
                        r#"
impl Revision for {} {{
    fn get_angelegt_von(&self) -> Option<String> {{
        self.angelegt_von.clone()
    }}
    fn set_angelegt_von(&mut self, von: &Option<String>) {{
        self.angelegt_von = von.clone();
    }}
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {{
        self.angelegt_am
    }}
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {{
        self.angelegt_am = am.clone();
    }}
    fn get_geaendert_von(&self) -> Option<String> {{
        self.geaendert_von.clone()
    }}
    fn set_geaendert_von(&mut self, von: &Option<String>) {{
        self.geaendert_von = von.clone();
    }}
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {{
        self.geaendert_am
    }}
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {{
        self.geaendert_am = am.clone();
    }}
}}
"#,
                        t.name.to_upper_camel_case(),
                    )
                    .as_str(),
                );
            }
        }
        sb.to_string()
    }

    /// Returns diesel type of column.
    fn get_diesel_type(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "Integer",
            "VARCHAR" => "Text",
            "DATE" => "Date",
            "TIMESTAMP" => "Timestamp",
            "BOOLEAN" => "Bool",
            "DECIMAL(21,4)" => "Double",
            "BLOB" => "Binary",
            _ => c.type_.as_str(),
        };
        if c.nullable {
            return format!("Nullable<{}>", t);
        }
        t.to_string()
    }

    /// Returns rust type of column.
    fn get_rust_type(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "i32",
            "VARCHAR" => "String",
            "DATE" => "NaiveDate",
            "TIMESTAMP" => "NaiveDateTime",
            "BOOLEAN" => "bool",
            "DECIMAL(21,4)" => "f64",
            "BLOB" => "Vec<u8>",
            _ => c.type_.as_str(),
        };
        if c.nullable {
            return format!("Option<{}>", t);
        }
        if c.length < 0 || c.extension {
            self::mach_nichts();
        }
        t.to_string()
    }

    /// Returns clone function of column.
    fn get_rust_type_clone(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "",
            "VARCHAR" => ".clone()",
            "DATE" => ".clone()",
            "TIMESTAMP" => ".clone()",
            "BOOLEAN" => "",
            "DECIMAL(21,4)" => "",
            "BLOB" => ".clone()",
            _ => c.type_.as_str(),
        };
        t.to_string()
    }

    /// Returns as_ref function of column.
    fn get_rust_type_as_ref(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "",
            "VARCHAR" => self::iif(c.nullable, ".as_ref()", ".as_str()"),
            "DATE" => "",      //".clone()",
            "TIMESTAMP" => "", //".clone()",
            "BOOLEAN" => "",
            "DECIMAL(21,4)" => "",
            "BLOB" => "", //".clone()",
            _ => c.type_.as_str(),
        };
        t.to_string()
    }

    /// Reads file tables.xml.
    fn read_tables(tables: &mut Vec<Table>) {
        let tables_src = include_str!("/home/wolfgang/cs/csbp/CSBP/Resources/Tables.xml");
        let mut reader = Reader::from_str(tables_src);
        reader.trim_text(true);

        let mut data = false;
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    QName(b"table") => {
                        let mut a = e.attributes().filter(|a| {
                            std::str::from_utf8(a.as_ref().unwrap().key.local_name().as_ref())
                                .unwrap()
                                == "name"
                        });
                        if let Some(Ok(a2)) = a.next() {
                            let key = a2.value;
                            let t = Table {
                                name: std::str::from_utf8(&key).unwrap().to_string(),
                                columns: Vec::new(),
                            };
                            // println!("table {}", t.name);
                            tables.push(t);
                            data = true;
                        }
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name() {
                    QName(b"column") => {
                        let atts = e
                            .attributes()
                            .map(|a| {
                                let at = a.unwrap();
                                let key = at.key.local_name();
                                let key0 = key.as_ref().clone();
                                let key1 = std::str::from_utf8(key0).unwrap().to_string();
                                let key2 = Box::new(key1);
                                (key2, std::str::from_utf8(&at.value).unwrap().to_string())
                            })
                            .collect::<Vec<_>>();
                        let mut c = Column {
                            name: get_attribut_value(&atts, "name"),
                            type_: get_attribut_value(&atts, "type"),
                            length: self::to_i32(get_attribut_value(&atts, "length").as_str()),
                            nullable: get_attribut_value(&atts, "nullable") == "true",
                            extension: get_attribut_value(&atts, "extension") == "true",
                            revision: false,
                            primary_key: false,
                        };
                        if c.name.to_lowercase().starts_with("angelegt_")
                            || c.name.to_lowercase().starts_with("geaendert_")
                            || c.name.to_lowercase().starts_with("replikation_uid")
                        {
                            c.revision = true;
                        }
                        //     // println!(
                        //     //     "  column {} {} {} {}",
                        //     //     c.name, c.type_, c.nullable, c.primary_key
                        //     // );
                        if let Some(t) = tables.last_mut() {
                            t.columns.push(c);
                        }
                    }
                    QName(b"keycolumn") => {
                        if let Some(Ok(a)) = e.attributes().into_iter().next() {
                            if let Ok(name) = std::str::from_utf8(&a.value) {
                                if let Some(t) = tables.last_mut() {
                                    if let Some(c) =
                                        t.columns.iter_mut().filter(|a| a.name == name).next()
                                    {
                                        // println!("    keycolumn {}", c.name);
                                        c.primary_key = true;
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                },
                Ok(Event::Text(_e)) => {
                    if data {
                        // let v = e.unescape_and_decode(&reader).unwrap();
                        // sb.push_str(v.as_str());
                        // sb.push_str("\"#,\n");
                        // data = false;
                        // println!("value: {:?}", v);
                    }
                }
                Ok(Event::End(ref e)) => match e.name() {
                    QName(b"table") => {
                        data = false;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other events we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
    }

    fn get_attribut_value(atts: &Vec<(Box<String>, String)>, key: &str) -> String {
        let keybox = Box::new(key.to_string());
        if let Some(att) = atts.iter().filter(|a| a.0 == keybox).next() {
            return att.1.to_string();
        }
        "".into()
    }

    /// The function does nothing and always returns 0.
    fn mach_nichts() -> i32 {
        0
    }

    /// Converts string to i32.
    /// * s: Affected string.
    pub fn to_i32(s: &str) -> i32 {
        let x = s.parse::<i32>();
        if let Ok(i) = x {
            return i;
        }
        0
    }

    /// Returns string in dependency of the bool value.
    pub fn iif<'a>(b: bool, strue: &'a str, sfalse: &'a str) -> &'a str {
        if b {
            return strue;
        }
        sfalse
    }
}
