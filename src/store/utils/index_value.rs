use tantivy::schema;
use tantivy::TantivyDocument;

pub fn index_facet(mut document: TantivyDocument, field: schema::Field, value: String) -> TantivyDocument {
    let mut facet_value = value.to_owned();
    if !facet_value.starts_with("/") {
        facet_value = "/".to_owned();
        facet_value.push_str(value.as_str());
    }

    document.add_facet(field, schema::Facet::from(facet_value.as_str()));
    document
}

pub fn index_optional_facet(document: TantivyDocument, field: schema::Field, value: Option<String>) -> TantivyDocument {
    match value {
        Some(value) => {
            index_facet(document, field, value)
        },
        None => {
            document
        },
    }
}

pub fn index_optional_text(mut document: TantivyDocument, field: schema::Field, value: Option<String>) -> TantivyDocument {
    match value {
        Some(value) => {
            document.add_text(field, value);
        },
        None => {},
    }

    document
}

pub fn index_optional_i64(mut document: TantivyDocument, field: schema::Field, value: Option<i64>) -> TantivyDocument {
    match value {
        Some(value) => {
            document.add_i64(field, value);
        },
        None => {},
    }

    document
}
