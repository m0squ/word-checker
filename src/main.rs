use std::fs::{ File, read_to_string };
use std::io::{ stdin, Write };

fn main()
{
    println!("Enter input file path:");
    let mut input_path = String::new();
    stdin().read_line(&mut input_path).expect("Error while reading user input");
    let input_content = read_to_string(input_path.trim()).expect("Error while reading file content");
    
    println!("Enter output file path:");
    let mut output_path = String::new();
    stdin().read_line(&mut output_path).expect("Error while reading user input");

    println!("Enter the length that the words should have:");
    let mut length = String::new();
    stdin().read_line(&mut length).expect("Error while reading user input");

    let mut accepted_words = Vec::new();
    for line in input_content.lines()
    {
        if line.chars().count() == length.trim().parse::<usize>().unwrap()
        {
            accepted_words.push(line);
        }
    }
    let mut output_file = File::create(output_path.trim()).expect("Error while creating output file");
    for word in accepted_words
    {
        writeln!(output_file, "{word}").expect("Error while writing to output file");
    }
    println!("Output file generated succesfully.")
}
