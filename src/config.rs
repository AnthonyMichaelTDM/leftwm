use super::manager::Manager;
use super::utils::screen::Screen;
use super::utils::workspace::Workspace;
use super::display_servers::DisplayServer;

pub fn load_config<T: DisplayServer>(manager: &mut Manager<T> ){


    // default to tags 1 to 9
    for i in 1..10 {
        manager.tags.push( i.to_string() );
    }

    // defaults is to build one workspace per screen
    if manager.workspaces.len() == 0 {
        for s in manager.screens.clone() {
            manager.workspaces.push( Workspace::from_screen(s) )
        }
    }

    //default the workspaces to show first tags in order
    let mut i = 0;
    let workspaces = &mut manager.workspaces;
    for ws in workspaces {
        let t = manager.tags[i].clone();
        ws.show_tag( t );
        i = i + 1;
    }

}




#[test]
fn default_config_should_create_tags_1_to_9(){
    let mut subject = Manager::new();
    let screens: Vec<Screen> = Vec::new();
    load_config(&mut subject, screens);
    let tags = subject.tags.clone();
    assert!( tags[0] == "1", "default tag {1} did not load");
    assert!( tags[1] == "2", "default tag {2} did not load");
    assert!( tags[2] == "3", "default tag {3} did not load");
    assert!( tags[3] == "4", "default tag {4} did not load");
    assert!( tags[4] == "5", "default tag {5} did not load");
    assert!( tags[5] == "6", "default tag {6} did not load");
    assert!( tags[6] == "7", "default tag {6} did not load");
    assert!( tags[7] == "8", "default tag {7} did not load");
    assert!( tags[8] == "9", "default tag {8} did not load");
}


#[test]
fn default_config_should_be_one_workspace_per_screen(){
    let mut subject = Manager::new();
    let mut screens = Vec::new();
    let x: Screen = unsafe{ std::mem::zeroed() };
    screens.push(x);
    load_config(&mut subject, screens);
    assert!( subject.screens.len() == subject.workspaces.len(), "default workspaces did not load");
}

#[test]
fn after_loading_config_it_should_know_about_all_screens(){
    let mut subject = Manager::new();
    let mut screens = Vec::new();
    let x: Screen = unsafe{ std::mem::zeroed() };
    screens.push(x);
    load_config(&mut subject, screens);
    assert!( subject.screens.len() == 1, "Was unable to manage the screen");
}
