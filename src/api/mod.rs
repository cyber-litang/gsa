pub mod list;
pub mod all;

use std::time::Duration;

use anyhow::Result;

pub async fn list_all_students(site_id: String) -> Result<Vec<(String, Vec<String>)>> {
    let assignments = list::list(site_id).await?;
    let mut result = Vec::new();
    for assignment in assignments {
        let students: Vec<_> = all::all(assignment.id).await?.into_iter().map(|v| v.account).collect();
        result.push((assignment.title, students));
        tokio::time::sleep(Duration::from_millis(400)).await;
    }
    Ok(result)
}