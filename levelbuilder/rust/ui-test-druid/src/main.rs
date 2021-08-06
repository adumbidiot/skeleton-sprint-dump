use druid::{UiState, UiMain};
use druid::widget::{Column, EventForwarder, KeyListener, Label, Padding, Row};
use druid_shell::platform::WindowBuilder;
use druid_shell::win_main;

fn main() {
    druid_shell::init();

    let mut run_loop = win_main::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let mut state = UiState::new();
	
	let display = Label::new("HELPME".to_string()).ui(&mut state);
	let button = Row::new().ui(&[display], &mut state);
	
	builder.set_handler(Box::new(UiMain::new(state)));
	builder.set_title("HELP");

    let window = builder.build().expect("built window");
	window.show();
    run_loop.run();
}
