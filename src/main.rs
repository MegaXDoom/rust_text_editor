use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use termion::{clear, cursor};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{stdout, stdin, Read};

fn main() {
    //set terminal to raw mode

    //create editor
    let mut editor = Editor::new();
    //display the buffer
    editor.display();
    
    //loop steping when there is user inputs using termion raw mode

     

    //save the buffer
    editor.save();

}

//structure for the text editor
struct Editor {
    //standard out
    stdout: RawTerminal<std::io::Stdout>,
    //the current line number
    line: u32,
    //the current column number
    column: u32,
    //the text buffer
    buffer: Vec<String>,
    //the maximum number of lines in the buffer
    max_line: u32,
    //the maximum number of columns in the buffer
    max_column:Vec<u32>,
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
        let max_line = buffer.len() as u32;
        let mut max_column = Vec::new();
        for line in &buffer{
            max_column.push(line.len() as u32);
        }
        //initialize the cursor position
        let line = 0;
        let column = 0;
        //i like it raw
        let stdout = stdout().into_raw_mode().unwrap();
        //return the editor
        Editor {
            stdout, 
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
        let in_virtical_bound = self.line-1 > 0 && self.line-1 < self.max_line;
        let in_horizontal_bound = self.column > 0 && self.column < self.max_column[(self.line-1) as usize];
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
        let in_virtical_bound = self.line+1 > 0 && self.line+1 < self.max_line;
        let in_horizontal_bound = self.column > 0 && self.column < self.max_column[(self.line+1) as usize];
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
        let in_horizontal_bound = self.column-1 > 0 && self.column-1 < self.max_column[(self.line) as usize];
        let in_virtical_bound = self.line-1 > 0 && self.line-1 < self.max_line;
        if in_horizontal_bound{
            self.column -= 1;
        } else if in_virtical_bound{
            self.move_up();
            self.column = self.max_column[(self.line) as usize];
        }
    }
    //function to move the cursor right
    fn move_right(&mut self){}
    //function to insert a character
    fn insert(&mut self, c: char){}
    //function to delete a character
    fn delete(&mut self){}
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
        print!("{}", termion::clear::All);
        termion::cursor::Goto(self.column as u16,self.line as u16);
        //iterate over the lines in the buffer
        for line in &self.buffer{
            //print the line
            println!("{}", line);
        }
    }
} 

