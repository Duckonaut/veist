use api::client::send_get;
use env_logger;
use structopt::StructOpt;
use tokio;

mod api;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "veist",
    about = "Versatile External Item and Stat Tool for Destiny 2",
    long_about = "Versatile External Item and Stat Tool for Destiny 2\n\n\x1b[0m 'VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV' ,V\x1b[92m VVVVVVVVVVV           ,VVVVVVVVVVVVV' \n\x1b[0m    'VVVVVVVVVVVVVVVVVVVVVVVVVVV' ,VVVV\x1b[92m VVVVVVVVVVV        ,VVVVVVVVVVVVV'    \n\x1b[0m       'VVVVVVVVVVVVVVVVVVVVV' ,VVVVVVV\x1b[92m VVVVVVVVVVV     ,VVVVVVVVVVVVV'       \n\x1b[0m          'VVVVVVVVVVVVVVV' ,VVVVVVVVVV\x1b[92m VVVVVVVVVVV  ,VVVVVVVVVVVVV'          \n\x1b[0m             'VVVVVVVVV' ,VVVVVVVVVVVVV\x1b[92m VVVVVVVVVVV,VVVVVVVVVVVV'             \n\x1b[0m                'VVV' ,VVVVVVVVVVVVVVVV\x1b[92m VVVVVVVVVVVVVVVVVVVVV'                \n\x1b[0m                    'VVVVVVVVVVVVVVVVVV\x1b[92m VVVVVVVVVVVVVVVVVV'                   \n\x1b[0m                      'VVVVVVVVVVVVVVVV\x1b[92m VVVVVVVVVVVVVVV'                      \n\x1b[0m                         'VVVVVVVVVVVVV\x1b[92m VVVVVVVVVVVV'                         \n\x1b[0m                            'VVVVVVVVVV\x1b[92m VVVVVVVVV'                            \n\x1b[0m                               'VVVVVVV\x1b[92m VVVVVV'                               \n\x1b[0m                                  'VVVV\x1b[92m VVV'                                  \n\x1b[0m                                     'V\x1b[92m '                                     \x1b[0m\n"
)]

enum Args {
    Weapon { name: String },
    Armor { name: String },
    Character { class: String },
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::from_args();

    println!("{}", send_get("App/FirstParty").await);
}
