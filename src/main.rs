use std::fs::File;
use std::io::{BufRead, BufReader, stdout};
use std::io::Write;
use crossterm::{execute, terminal, cursor, event, style, Result};
use crossterm::event::read;

fn main() {
    //set terminal to raw mode
    crossterm::terminal::enable_raw_mode().expect("can run in raw mode");
    //create editor
    let mut editor = Editor::new();
    //display the buffer
    editor.display();

    
    //loop steping when there is user inputs using crossterm raw mode
    loop {
        execute!(stdout(), cursor::MoveTo(editor.column as u16, editor.line as u16)).unwrap();
        //read the user input
        let input = read().unwrap();
        //match the user input
        match input {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Char(c) => {
                        editor.insert(c);
                    }
                    event::KeyCode::Esc => {
                        break;
                    }
                    event::KeyCode::Up => {
                        editor.move_up();
                    }
                    event::KeyCode::Down => {
                        editor.move_down();
                    }
                    event::KeyCode::Left => {
                        editor.move_left();
                    }
                    event::KeyCode::Right => {
                        editor.move_right();
                    }
                    event::KeyCode::Backspace => {
                        editor.delete();
                    }
                    event::KeyCode::Enter => {
                        editor.newline();
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        editor.display();

        

    } 

    //save the buffer
    editor.save();

}

//structure for the text editor
struct Editor {
    //the current line number
    line: i32,
    //the current column number
    column: i32,
    //the text buffer
    buffer: Vec<String>,
    //the maximum number of lines in the buffer
    max_line: i32,
    //the maximum number of columns in the buffer
    max_column:Vec<i32>,
    //the file path
    path: String,
}
//traits for the text editor
trait EditorTrait {
    //create new editor
    fn new() -> Self;
    //function to move the cursor up
    fn move_up(&mut self);
    //function to move the cursor down
    fn move_down(&mut self);
    //function to move the cursor left
    fn move_left(&mut self);
    //function to move the cursor right
    fn move_right(&mut self);
    //function to insert a character
    fn insert(&mut self, c: char);
    //function to delete a character
    fn delete(&mut self);
    //function to save the buffer to a file
    fn save(&self);
    //function to load a file into the buffer
    fn load(&mut self);
    //function to display the buffer
    fn display(&self);
    //at a newline character
    fn newline(&mut self);
}

//implementation of the EditorTrait for the Editor structure
impl EditorTrait for Editor {
    //create new editor
    fn new() -> Self {
        //get file path from command line arguments
        let args: Vec<String> = std::env::args().collect();
        let path = args[1].clone();
        //initialize the buffer
        let mut buffer = Vec::new();
        //open the file
        let file = File::open(&path).expect("File not found");
        //read the file
        let reader = BufReader::new(file);
        for line in reader.lines(){
            //add the line to the buffer
            buffer.push(line.unwrap());
        }
        //initialize the maximum number of lines and columns
        let max_line = buffer.len() as i32;
        let mut max_column = Vec::new();
        for line in &buffer{
            max_column.push(line.len() as i32 +1);
        }
        //initialize the cursor position
        let line = 0;
        let column = 0;
        //return the editor
        Editor {
            line,
            column,
            buffer,
            max_line,
            max_column,
            path,
        }
    }
    //function to move the cursor up
    fn move_up(&mut self){
        //check if the cursor is in the bounds of the buffer
        let in_virtical_bound = self.line+1 >= 0 && self.line+1 < self.max_line;
        let mut in_horizontal_bound = false;
        if in_virtical_bound{
            in_horizontal_bound = self.column >= 0 && self.column < self.max_column[(self.line-1) as usize];
        }
        //move the cursor up
        if in_virtical_bound{
            if in_horizontal_bound{
                self.line -= 1;
            }else{
                self.column = self.max_column[(self.line-1) as usize];
                self.line -= 1;
            }
        }
    }
    //function to move the cursor down
    fn move_down(&mut self){
        //check if the cursor is in the bounds of the buffer
        let in_virtical_bound = self.line+1 >= 0 && self.line+1 < self.max_line-1;
        let mut in_horizontal_bound = false;
        if in_virtical_bound{
            in_horizontal_bound = self.column >= 0 && self.column < self.max_column[(self.line+1) as usize];
        }
        //move the cursor down
        if in_virtical_bound{
            if in_horizontal_bound{
                self.line += 1;
            }else{
                self.column = self.max_column[(self.line+1) as usize];
                self.line += 1;
            }
        }
    }
    //function to move the cursor left
    fn move_left(&mut self){
        let in_horizontal_bound = self.column-1 >= 0 && self.column-1 < self.max_column[(self.line) as usize];
        let in_virtical_bound = self.line-1 >= 0 && self.line-1 < self.max_line;
        if in_horizontal_bound{
            self.column -= 1;
        } else if in_virtical_bound{
            self.column = self.max_column[(self.line-1) as usize]-1;
            self.move_up();
        }
    }
    //function to move the cursor right
    fn move_right(&mut self){
        let in_horizontal_bound = self.column+1 >= 0 && self.column+1 < self.max_column[(self.line) as usize];
        let in_virtical_bound = self.line+1 >= 0 && self.line+1 < self.max_line;
        if in_horizontal_bound{
            self.column += 1;
        } else if in_virtical_bound{
            self.move_down();
            self.column = 0;
        }
    }
    //function to insert a character
    fn insert(&mut self, c: char){
        //check if the cursor is in the bounds of the buffer
        let in_virtical_bound = self.line >= 0 && self.line < self.max_line;
        let in_horizontal_bound = self.column >= 0 && self.column < self.max_column[(self.line) as usize];
        //insert the character
        if in_virtical_bound{
            if in_horizontal_bound{
                let line = self.buffer.get_mut(self.line as usize).unwrap();
                line.insert(self.column as usize, c);
                self.column += 1;
                self.max_column[(self.line) as usize] += 1;
            }else{
                let line = self.buffer.get_mut(self.line as usize).unwrap();
                line.push(c);
                self.column += 1;
                self.max_column[(self.line) as usize] += 1;
            }
        }
    }
    //function to delete a character
    fn delete(&mut self){
        //check if the cursor is in the bounds of the buffer
        let in_virtical_bound = self.line >= 0 && self.line < self.max_line;
        let in_horizontal_bound = self.column > 0 && self.column < self.max_column[(self.line) as usize];
        let is_first_char = self.column == 0;
        //delete the character
        if is_first_char{
            if self.line > 0{
                let mut lines = self.buffer.clone();
                let line = lines.remove(self.line as usize);
                let prev_line = lines.get_mut(self.line as usize - 1).unwrap();
                prev_line.push_str(&line);
                lines[self.line as usize - 1] = prev_line.clone();
                self.buffer = lines;
                self.line -= 1;
                self.max_line -= 1;
                self.column = self.max_column[(self.line) as usize]-1;
                self.max_column.remove(self.line as usize);
                self.max_column[self.line as usize] = self.buffer[self.line as usize].len() as i32 + 1;
                
            }
        }else{
            if in_virtical_bound{
                if in_horizontal_bound{
                    let line = self.buffer.get_mut(self.line as usize).unwrap();
                    line.remove(self.column as usize - 1);
                    self.column -= 1;
                    self.max_column[(self.line) as usize] -= 1;
                }else{
                    let line = self.buffer.get_mut(self.line as usize).unwrap();
                    line.pop();
                    self.column -= 1;
                    self.max_column[(self.line) as usize] -= 1;
                }
            }
        }
    }
    //function to save the buffer to a file
    fn save(&self){
        //open the file
        let mut file = File::create(&self.path).expect("File not found");
        //iterate over the lines in the buffer
        for line in &self.buffer{
            //write the line to the file
            writeln!(file, "{}", line).expect("Unable to write to file");
        }
    }
    //function to load a file into the buffer
    fn load(&mut self){
        //open the file
        let file = File::open(&self.path).expect("File not found");
        //read the file
        let reader = BufReader::new(file);
        //iterate over the lines in the file
        for line in reader.lines(){
            //add the line to the buffer
            self.buffer.push(line.unwrap());
        }
    }
    //function to display the buffer
    fn display(&self){
        //clear screen
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        //iterate over the lines in the buffer
        let mut i = 0;
        for line in &self.buffer{
            //print the line
            execute!(stdout(), cursor::MoveTo(0, i)).unwrap();
            print!("{} ", line);
            i += 1;
        }
    }
    //function to add a newline character
    fn newline(&mut self){
        let mut buf = self.buffer.clone();
        let line = buf.get(self.line as usize).unwrap().clone();
        let lines = line.split_at(self.column as usize);
        buf[self.line as usize] = lines.0.to_string();
        buf.insert(self.line as usize + 1, lines.1.to_string());
        self.buffer = buf;
        self.max_line += 1;
        self.line += 1;
        self.max_column.insert(self.line as usize, self.buffer[self.line as usize].len() as i32 + 1);
        self.max_column[self.line as usize - 1] = self.buffer[self.line as usize - 1].len() as i32 + 1;
        self.column = 0;

    }
} 

