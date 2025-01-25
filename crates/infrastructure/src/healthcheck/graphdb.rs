use domain::value_object::error::healthcheck::HealthCheckError;
use neo4rs::{Graph, query};

pub(super) async fn healthcheck_graphdb(graphdb: Graph) -> Result<(), HealthCheckError> {
    // test
    // get (item:Item {id: 1}) test
    let _ = graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await?;
    Ok(())
}
