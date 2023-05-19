use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "RNginx")]
#[command(author = "Aya0wind <ljj981018@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Configurable reverse proxy server ", long_about = None)]
pub struct Args {
    #[arg(default_value="rnginx_conf.yaml")]
    #[arg(short = 'c')]
    #[arg(long = "config-path")]
    pub config: String,
}
