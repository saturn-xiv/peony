extern crate flatbuffers;
extern crate peony;

use flatbuffers::FlatBufferBuilder;
use peony::protos::nut::{EmailTask, EmailTaskArgs};

#[test]
fn test_flatbuffers() {
    let to = "aaa@aaa.com";
    let subject = "sss";
    let body = "bbb";

    let mut builder = FlatBufferBuilder::new_with_capacity(1 << 10);
    let to = builder.create_string(to);
    let subject = builder.create_string(subject);
    let body = builder.create_string(body);
    let cc: Vec<&str> = Vec::new();
    let cc = builder.create_vector_of_strings(&cc);
    let bcc: Vec<&str> = Vec::new();
    let bcc = builder.create_vector_of_strings(&bcc);
    let email = EmailTask::create(
        &mut builder,
        &EmailTaskArgs {
            to: Some(to),
            subject: Some(subject),
            body: Some(body),
            cc: Some(cc),
            bcc: Some(bcc),
            ..Default::default()
        },
    );
    builder.finish(email, None);

    let buf = builder.finished_data();
    println!("generate {} bytes", buf.len());
    let task = flatbuffers::root::<EmailTask>(buf).unwrap();
    println!(
        "parse to: {} subject: {} body: {}",
        task.to(),
        task.subject(),
        task.body()
    );
}
