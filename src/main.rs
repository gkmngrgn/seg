use seg;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "seg")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(StructOpt)]
#[structopt(about = "Static Email Generator")]
enum Sub {
    /// Generates all available emails using [SOURCE] folder
    #[structopt(display_order = 1)]
    Generate {
        /// Source template folder
        #[structopt(short, long)]
        source: PathBuf,

        /// Output email folder
        #[structopt(short, long)]
        destination: PathBuf,
    },

    /// Sends a test mail to preview
    #[structopt(display_order = 2)]
    Send {
        /// Email template file
        #[structopt(short, long)]
        template: PathBuf,

        /// Email subject
        #[structopt(short, long)]
        subject: String,

        /// Receiver email
        #[structopt(short, long)]
        email: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Sub::Generate {
            source,
            destination,
        } => {
            let msg = match seg::template::generate_all_templates(source, destination) {
                Ok(_) => "All templates are generated successfully.".to_string(),
                Err(msg) => msg,
            };
            println!("{}", msg);
        }
        Sub::Send {
            template,
            subject,
            email,
        } => {
            println!("command is send.");
            println!(
                "template: {:?}, subject: {}, email: {}",
                template, subject, email
            );
        }
    }
}
