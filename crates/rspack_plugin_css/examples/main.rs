use temp_test_utils::TestOptions;
#[tokio::main]
async fn main() {
  let mut cur_dir = std::env::current_dir().unwrap();
  cur_dir = cur_dir.join("webpack_css_cases_to_be_migrated/at-import-in-the-middle");
  println!("{:?}", cur_dir);
  let options = TestOptions::from_fixture(&cur_dir).into();

  println!("{:?}", options);
  let mut compiler = rspack::rspack(options, Default::default());

  let _stats = compiler
    .run()
    .await
    .unwrap_or_else(|_| panic!("failed to compile in fixtrue {:?}", cur_dir));
  // println!("{:?}", stats);
}