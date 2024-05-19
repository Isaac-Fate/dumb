use rustyline::DefaultEditor;

fn main() {
    let mut editor = DefaultEditor::new().unwrap();

    loop {
        let readline = editor.readline("dumb>>> ");
        match readline {
            Ok(line) => {
                match line.as_str() {
                    r"\q" | r"\quit" | r"\exit" => {
                        break;
                    }
                    _ => {
                        // editor.add_history_entry(line.as_str());
                        println!("your input: {}", line);
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}
