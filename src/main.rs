use std::path::Path;

mod api;
mod headers;
mod writer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    headers::set_identity("".to_string());
    headers::set_authorization("".to_string());
    headers::set_blade_auth("".to_string());
    let result = api::list_all_students("1558997828460613635".to_string()).await?;
    println!(
        "Hello, world! {:#?}",
        result
    );
    writer::write_file(Path::new("r.csv"), result)?;
    Ok(())
}
