use fortune_generator::{error::Error, run};

fn main() {
    let result = run();

    if let Err(error) = result {
        match error {
            Error::ChecksFailed => std::process::exit(1),
            Error::FortuneSettingsLoadFailure(msg) => {
                println!("Fortune settings load failure: {}", msg);
                std::process::exit(2)
            }
            Error::PdfGenerateFailure(msg) => {
                println!("PDF generate failure: {}", msg);
                std::process::exit(3)
            }
        }
    }
}
