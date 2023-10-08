use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};
use clap::{Parser, Subcommand};

use Command::*;

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A command line tool to hide messages in png files"
)]
#[command(
    help_template = "{author} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
pub struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Encode {
        #[arg(help = "Path to input png file")]
        file_path: String,

        #[arg(help = "Type of Chunk to be encoded")]
        chunk_type: String,

        #[arg(help = "Message to be encoded")]
        message: String,

        #[arg(help = "Path where the encoded png will be written to")]
        output_file: Option<String>,
    },

    Decode {
        #[arg(help = "Path to input png file")]
        file_path: String,

        #[arg(help = "Type of Chunk to be decoded")]
        chunk_type: String,
    },

    Remove {
        #[arg(help = "Path to input png file")]
        file_path: String,

        #[arg(help = "Type of Chunk to be removed")]
        chunk_type: String,
    },

    Print {
        #[arg(help = "Path to input png file")]
        file_path: String,
    },
}

pub fn get_args() -> Result<Command> {
    let args = CliArgs::parse();
    Ok(args.command)
}

pub fn run(command: Command) -> Result<()> {
    dbg!(&command);
    match command {
        Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            let chunk = Chunk::new(
                ChunkType::from(
                    chunk_type
                        .as_bytes()
                        .try_into()
                        .expect("chunk_type length is not equal to 4"),
                ),
                message.into_bytes(),
            );
            let mut input = open(&file_path)?;
            let mut chunks: Vec<u8> = vec![];
            input.read_to_end(&mut chunks)?;

            let mut input_png = Png::try_from(chunks.as_slice())?;
            input_png.append_chunk(chunk);
            if let Some(output_file_path) = output_file {
                let f = File::create(output_file_path)?;
                let mut f = BufWriter::new(f);
                f.write_all(input_png.as_bytes().as_slice())?;
            }
        }
        Decode {
            file_path,
            chunk_type,
        } => {
            let mut input = open(&file_path)?;
            let mut chunks: Vec<u8> = vec![];
            input.read_to_end(&mut chunks)?;

            let input_png = Png::try_from(chunks.as_slice())?;
            match input_png.chunk_by_type(&chunk_type) {
                Some(chunk) => {
                    println!("{}", chunk);
                }
                None => {
                    return Err(format!("Chunk of type {} not found", chunk_type).into());
                }
            }
        }
        Remove {
            file_path,
            chunk_type,
        } => {
            let mut input = open(&file_path)?;
            let mut chunks: Vec<u8> = vec![];
            input.read_to_end(&mut chunks)?;

            let mut input_png = Png::try_from(chunks.as_slice())?;
            input_png.remove_chunk(&chunk_type)?;
            let f = File::create(file_path)?;
            let mut f = BufWriter::new(f);
            f.write_all(input_png.as_bytes().as_slice())?;
        }
        Print { file_path } => {
            let mut input = open(&file_path)?;
            let mut chunks: Vec<u8> = vec![];
            input.read_to_end(&mut chunks)?;
            let input_png = Png::try_from(chunks.as_slice())?;
            println!("{}", input_png)
        }
    }
    Ok(())
}
