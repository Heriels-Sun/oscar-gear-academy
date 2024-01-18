use gtest::{Program, System, Log};
use tamagotchi_io::*;

#[test]
fn smoke_test_01() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    _program.send(2, String::from("Heriel"));

    let _res = _program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Name("Heriel".to_string()));
    assert!(_res.contains(&expected_log));

    let _res   = _program.send(2, TmgAction::Age);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Age(0));
    assert!(_res.contains(&expected_log));

}

#[test]
fn interactions_test_02() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    _program.send(2, String::from("Sofia"));

    let _res = _program.send(2, TmgAction::Feed);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Fed);
    assert!(_res.contains(&expected_log));

    let _res = _program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Entertained);
    assert!(_res.contains(&expected_log));

    let _res = _program.send(2, TmgAction::Sleep);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Slept);
    assert!(_res.contains(&expected_log));

}

#[test]
fn owning_test_03() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    _program.send(2, String::from("Sofia"));

    let _res = _program.send(2, TmgAction::Approve(3.into()));
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Approved(3.into()));
    assert!(_res.contains(&expected_log));

    let _res = _program.send(2, TmgAction::Transfer(3.into()));
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Transferred(3.into()));
    assert!(_res.contains(&expected_log));

    let _res = _program.send(2, TmgAction::RevokeApproval);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::ApprovalRevoked);
    assert!(_res.contains(&expected_log));

}