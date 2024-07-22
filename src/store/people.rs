use tantivy::tokenizer::TokenFilter;
use tantivy::{
    TantivyDocument,
    TantivyError,
    directory,
    index,
    schema,
    tokenizer,
    doc,
};

use super::super::{
    config::INDEX_PEOPLE_PATH,
    models::person::Person,
    store::utils,
};

impl Person {
    pub async fn to_doc(self, schema: schema::Schema) -> Result<TantivyDocument, TantivyError> {
        let id_field = schema.get_field("id")?;
        let first_name_field = schema.get_field("first_name")?;
        let first_name_ngram_field = schema.get_field("first_name_ngram")?;
        let last_name_field = schema.get_field("last_name")?;
        let last_name_ngram_field = schema.get_field("last_name_ngram")?;
        let email_field = schema.get_field("email")?;
        let email_ngram_field = schema.get_field("email_ngram")?;
        let sex_field = schema.get_field("sex")?;
        let address_country_field = schema.get_field("address_country")?;
        let address_zip_code_field = schema.get_field("address_zip_code")?;
        let address_city_field = schema.get_field("address_city")?;
        let address_city_ngram_field = schema.get_field("address_city_ngram")?;
        let address_line_1_field = schema.get_field("address_line_1")?;
        let address_line_1_ngram_field = schema.get_field("address_line_1_ngram")?;
        let address_line_2_field = schema.get_field("address_line_2")?;
        let address_line_2_ngram_field = schema.get_field("address_line_2_ngram")?;

        let mut document = doc!(
            id_field => self.id,
            first_name_field => self.first_name.clone(),
            first_name_ngram_field => self.first_name,
            last_name_field => self.last_name.clone(),
            last_name_ngram_field => self.last_name,
            email_field => self.email.clone(),
            email_ngram_field => self.email,
        );

        let mut sex_facet = "/sex/".to_owned();
        sex_facet.push_str(self.sex.as_str());
        document.add_facet(sex_field, sex_facet.as_str());

        match self.address {
            Some(address) => {
                match address.country {
                    Some(value) => {
                        let mut country_facet = "/country/".to_owned();
                        country_facet.push_str(value.as_str());
                        document.add_facet(address_country_field, country_facet.as_str());
                    },
                    None => {},
                }

                document = utils::index_optional_text(document, address_zip_code_field, address.zip_code);
                document = utils::index_optional_text(document, address_city_field, address.city.clone());
                document = utils::index_optional_text(document, address_city_ngram_field, address.city);
                document = utils::index_optional_text(document, address_line_1_field, address.line_1.clone());
                document = utils::index_optional_text(document, address_line_1_ngram_field, address.line_1);
                document = utils::index_optional_text(document, address_line_2_field, address.line_2.clone());
                document = utils::index_optional_text(document, address_line_2_ngram_field, address.line_2);
            },
            None => {},
        }

        Ok(document)
    }
}

pub async fn open_index(schema: schema::Schema) -> Result<index::Index, TantivyError> {
    let directory = directory::MmapDirectory::open(INDEX_PEOPLE_PATH.clone())?;
    let index = index::Index::open_or_create(directory, schema)?;

    let ngram_2_4_tokenizer = tokenizer::NgramTokenizer::new(2, 4, false)?;
    let simple_tokenizer = tokenizer::LowerCaser.transform(
        tokenizer::SimpleTokenizer::default(),
    );

    index.tokenizers().register("ngram_2_4", ngram_2_4_tokenizer);
    index.tokenizers().register("simple", simple_tokenizer);

    Ok(index)
}

pub fn build_schema() -> schema::Schema {
    let mut schema_builder = schema::SchemaBuilder::new();

    let ngram_2_4_field_options = schema::TextOptions::default()
        .set_indexing_options(
            schema::TextFieldIndexing::default()
                .set_tokenizer("ngram_2_4")
                .set_index_option(schema::IndexRecordOption::WithFreqsAndPositions),
        );

    let simple_field_options = schema::TextOptions::default()
        .set_indexing_options(
            schema::TextFieldIndexing::default()
                .set_tokenizer("simple")
                .set_index_option(schema::IndexRecordOption::WithFreqsAndPositions),
        );

    schema_builder.add_text_field("id", schema::STRING | schema::STORED);

    schema_builder.add_text_field("first_name", simple_field_options.clone());
    schema_builder.add_text_field("first_name_ngram", ngram_2_4_field_options.clone());

    schema_builder.add_text_field("last_name", simple_field_options.clone());
    schema_builder.add_text_field("last_name_ngram", ngram_2_4_field_options.clone());

    schema_builder.add_facet_field("sex", schema::FacetOptions::default());

    schema_builder.add_text_field("email", simple_field_options.clone());
    schema_builder.add_text_field("email_ngram", ngram_2_4_field_options.clone());

    schema_builder.add_facet_field("address_country", schema::FacetOptions::default());

    schema_builder.add_text_field("address_zip_code", simple_field_options.clone());

    schema_builder.add_text_field("address_city", simple_field_options.clone());
    schema_builder.add_text_field("address_city_ngram", ngram_2_4_field_options.clone());

    schema_builder.add_text_field("address_line_1", simple_field_options.clone());
    schema_builder.add_text_field("address_line_1_ngram", ngram_2_4_field_options.clone());

    schema_builder.add_text_field("address_line_2", simple_field_options.clone());
    schema_builder.add_text_field("address_line_2_ngram", ngram_2_4_field_options.clone());

    schema_builder.build()
}
