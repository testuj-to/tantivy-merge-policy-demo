use std::{
    fs,
    thread,
    time::{
        Instant,
        Duration,
    },
};
use serde::{
    Serialize,
    Deserialize,
};
use serde_json;
use tantivy::{
    Index,
    IndexWriter,
};

use tantivy_merge_policy_demo::{
    config,
    models,
    store,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RunResult {
    total_index_time: String,
    final_segment_file_counts: SegmentFileCounts,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SegmentFileCounts {
    fast: u32,
    fieldnorm: u32,
    idx: u32,
    pos: u32,
    store: u32,
    term: u32,
}

async fn get_index() -> Index {
    match fs::remove_dir_all(config::INDEX_PEOPLE_PATH.clone()) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to cleanup index directory: {}", err);
        },
    }

    match fs::create_dir_all(config::INDEX_PEOPLE_PATH.clone()) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to prepare index directory: {}", err);
        },
    }

    match store::people::open_index(store::PERSON_SCHEMA.clone()).await {
        Ok(index) => index,
        Err(err) => {
            panic!("Failed to open people index: {:?}", err);
        },
    }
}

fn count_segment_file_counts() -> Result<SegmentFileCounts, std::io::Error> {
    let directory = fs::read_dir(config::INDEX_PEOPLE_PATH.clone())?;

    let mut segment_file_counts = SegmentFileCounts{
        fast: 0,
        fieldnorm: 0,
        idx: 0,
        pos: 0,
        store: 0,
        term: 0,
    };

    for entry in directory {
        match entry {
            Ok(entry) => {
                match entry.file_name().to_str() {
                    Some(file_name) => {
                        if file_name.ends_with("fast") {
                            segment_file_counts.fast += 1;
                        } else if file_name.ends_with("fieldnorm") {
                            segment_file_counts.fieldnorm += 1;
                        } else if file_name.ends_with("idx") {
                            segment_file_counts.idx += 1;
                        } else if file_name.ends_with("pos") {
                            segment_file_counts.pos += 1;
                        } else if file_name.ends_with("store") {
                            segment_file_counts.store += 1;
                        } else if file_name.ends_with("term") {
                            segment_file_counts.term += 1;
                        }
                    },
                    None => {},
                };
            },
            Err(err) => {
                println!("Failed to read dir entry: {}", err)
            },
        }
    }

    Ok(segment_file_counts)
}

#[tokio::main]
async fn main() {
    let people_data = match fs::read(config::DATA_PEOPLE_PATH.clone()) {
        Ok(people_data) => people_data,
        Err(err) => {
            panic!("Failed to read the data: {}", err);
        },
    };

    let people: Vec<models::person::Person> = match serde_json::from_slice(&people_data) {
        Ok(people) => people,
        Err(err) => {
            panic!("Failed to parse the data: {}", err);
        },
    };

    /*
     * Test A
     *   - Single final commit / Merge policy: MergeWhenever / No waiting for merging
     */
    let result_a = run_a(people.clone()).await;
    println!("{}", serde_json::json!(result_a));

    /*
     * Test B
     *   - Single final commit / Merge policy: MergeWhenever / Wait for merging
     */
    // let result_b = run_b(people.clone()).await;
    // println!("{}", serde_json::json!(result_b));

    /*
     * Test C
     *   - Single final commit / Merge policy: TargetDocs / No waiting for merging
     */
    // let result_c = run_c(people.clone()).await;
    // println!("{}", serde_json::json!(result_c));

    /*
     * Test D (Infinite loop!)
     *   - Single final commit / Merge policy: TargetDocs / Wait for merging
     */
    // let result_d = run_d(people.clone()).await;
    // println!("{}", serde_json::json!(result_d));

    /*
     * Test E
     *   - Commit after every add_document / Merge policy: MergeWhenever / No waiting for merging
     */
    // let result_e = run_e(people.clone()).await;
    // println!("{}", serde_json::json!(result_e));

    /*
     * Test F
     *   - Commit after every add_document / Merge policy: MergeWhenever / Wait for merging
     */
    // let result_f = run_f(people.clone()).await;
    // println!("{}", serde_json::json!(result_f));

    /*
     * Test G
     *   - Commit after every add_document / Merge policy: TargetDocs / No waiting for merging
     */
    // let result_g = run_g(people.clone()).await;
    // println!("{}", serde_json::json!(result_g));

    /*
     * Test H (Infinite loop!)
     *   - Commit after every add_document / Merge policy: TargetDocs / Wait for merging
     */
    // let result_h = run_h(people).await;
    // println!("{}", serde_json::json!(result_h));

    println!("All done!! 🎉🎉🎉")
}

/*
 * Test A
 *
 *   - Single final commit
 *   - Merge policy: MergeWhenever
 *   - No waiting for merging
 */
async fn run_a(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::MergeWheneverPossiblePolicy::new("a".to_string());
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }
    }

    match writer.commit() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to commit the writer: {}", err);
        },
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test B
 *
 *   - Single final commit
 *   - Merge policy: MergeWhenever
 *   - Wait for merging
 */
async fn run_b(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::MergeWheneverPossiblePolicy::new("b".to_string());
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }
    }

    match writer.commit() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to commit the writer: {}", err);
        },
    }

    match writer.wait_merging_threads() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to wait for merging threads: {}", err);
        },
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test C
 *
 *   - Single final commit
 *   - Merge policy: TargetDocs
 *   - No waiting for merging
 */
async fn run_c(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::TargetDocsPerSegmentPolicy::new("c".to_string(), 10000);
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }
    }

    match writer.commit() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to commit the writer: {}", err);
        },
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test D (Infinite loop!)
 *
 *   - Single final commit
 *   - Merge policy: TargetDocs
 *   - Wait for merging
 */
async fn run_d(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::TargetDocsPerSegmentPolicy::new("d".to_string(), 10000);
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }
    }

    match writer.commit() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to commit the writer: {}", err);
        },
    }

    match writer.wait_merging_threads() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to wait for merging threads: {}", err);
        },
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test E
 *
 *   - Commit after every add_document
 *   - Merge policy: MergeWhenever
 *   - No waiting for merging
 */
async fn run_e(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::MergeWheneverPossiblePolicy::new("e".to_string());
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }

        match writer.commit() {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to commit the writer: {}", err);
            },
        }
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test F
 *
 *   - Commit after every add_document
 *   - Merge policy: MergeWhenever
 *   - Wait for merging
 */
async fn run_f(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::MergeWheneverPossiblePolicy::new("f".to_string());
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }

        match writer.commit() {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to commit the writer: {}", err);
            },
        }
    }

    match writer.wait_merging_threads() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to wait for merging threads: {}", err);
        },
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test G
 *
 *   - Commit after every add_document
 *   - Merge policy: TargetDocs
 *   - No waiting for merging
 */
async fn run_g(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::TargetDocsPerSegmentPolicy::new("g".to_string(), 10000);
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }

        match writer.commit() {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to commit the writer: {}", err);
            },
        }
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}

/*
 * Test H (Infinite loop!)
 *
 *   - Commit after every add_document
 *   - Merge policy: TargetDocs
 *   - Wait for merging
 */
async fn run_h(people: Vec<models::person::Person>) -> RunResult {
    thread::sleep(Duration::from_secs(5));

    let index = get_index().await;

    let start_instant = Instant::now();

    let mut writer: IndexWriter = match index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Failed to init writer: {}", err);
        },
    };

    let merge_policy = store::utils::TargetDocsPerSegmentPolicy::new("h".to_string(), 10000);
    writer.set_merge_policy(merge_policy.as_box());

    for person in people {
        let document = match person.to_doc(store::PERSON_SCHEMA.clone()).await {
            Ok(document) => document,
            Err(err) => {
                panic!("Failed to convert person into document: {}", err);
            },
        };

        match writer.add_document(document) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to add document to writer: {}", err);
            },
        }

        match writer.commit() {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed to commit the writer: {}", err);
            },
        }
    }

    match writer.wait_merging_threads() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to wait for merging threads: {}", err);
        },
    }

    RunResult{
        total_index_time: format!("{:?}", start_instant.elapsed()),
        final_segment_file_counts: match count_segment_file_counts() {
            Ok(segment_file_counts) => segment_file_counts,
            Err(err) => {
                panic!("Failed to count final segment files: {}", err);
            },
        },
    }
}
