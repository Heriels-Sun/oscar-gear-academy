use gtest::{Program, System, Log};
use tamagotchi_factory_io::*;

#[test]
fn factory_test_01() {
    let sys = System::new();
    let tamagotchi_code_id = sys.submit_code("../../Tamagotchi-01/target/wasm32-unknown-unknown/release/tamagotchi_01.opt.wasm");
    let tamagotchi_factory = Program::current(&sys);
    let res = tamagotchi_factory.send(100, tamagotchi_code_id);

    assert!(!res.main_failed());

    let _res = tamagotchi_factory.send(2, FactoryAction::CreateTamagotchi { tamagotchi_name: "Oscar".to_string() });
    let expected_log = Log::builder()
    .dest(2)
    .payload(FactoryEvent::TamagotchiCreated { tamagotchi_id: Default::default(), tamagotchi_address: Default::default() });
    assert!(_res.contains(&expected_log));

}