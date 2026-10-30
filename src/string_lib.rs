use crate::DubiousDubsState;



pub fn set_selected_dub_id(state: &mut DubiousDubsState, story_id: &String, hashcode: &String) {
state.selected_dub_id = format!("{}_{}", story_id, hashcode);
}