mod test;
pub(crate) mod db;

pub(crate) async fn run_all_tests()
{
    use test::*;

    test_speed(0).await;
}