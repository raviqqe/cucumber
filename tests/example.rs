use cucumber::{StatsWriter as _, World, then};

#[then(expr = "{string} is {string}")]
fn is(_: &mut TestWorld, x: String, y: String) -> Result<(), &'static str> {
    assert_eq!(x, y);

    Ok(())
}

#[derive(Clone, Copy, Debug, Default, World)]
struct TestWorld;

// Escaping values in example tables.
// https://cucumber.io/docs/gherkin/reference/#table-cell-escaping
#[tokio::test]
async fn example() {
    let writer = TestWorld::cucumber()
        .with_default_cli()
        .run("tests/features/example")
        .await;

    assert_eq!(writer.passed_steps(), 3);
}
