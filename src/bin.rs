fn main() {
    // let string = format!(
    //     "Hello world!\nYou are editing this file in '{}'.",
    //     edit::get_editor()
    //         .expect("can't find an editor")
    //         .to_str()
    //         .unwrap()
    // );
    // let edited = edit::edit(string).expect("editing failed");

    edit::edit_file_line_nr("test.txt", 5);

    // println!("after editing:\n{}", edited);
}
