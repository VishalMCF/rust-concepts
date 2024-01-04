mod player;

fn main() {
    player::cricket_player();
    clean::perform_clean();
    clean::files::clean_leaves();
}


mod clean {
    pub fn perform_clean(){
        println!("Perform cleaning of ground....");
    }

    pub mod files {
        pub fn clean_leaves(){
            println!("Removing dried and fallen leaves from the ground...");
        }
    }
}