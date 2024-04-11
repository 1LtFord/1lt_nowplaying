mod mpris_mediaplayer2;
mod environment;
mod nowplaying;
mod config;
mod web_display;

use environment::WorkingEnvironment;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    print_agpl_v3_disclaimer(version);
    print_external_librarys();

    let config = config::Config::new();
    
    let work_env = WorkingEnvironment::new(config.clone());

    let nowplaying = nowplaying::Service::new(&config, work_env);
    nowplaying.start();

}





fn print_agpl_v3_disclaimer(version: &str) {
    println!("1Lt-NowPlaying {} (AGPL 3.0)", version);
    println!("----------------------------------------------------------------------------");
    println!("This program is free software: you can redistribute it and/or modify");
    println!("it under the terms of the GNU Affero General Public License as published");
    println!("by the Free Software Foundation, either version 3 of the License, or");
    println!("(at your option) any later version.\n");
    println!("This program is distributed in the hope that it will be useful,");
    println!("but WITHOUT ANY WARRANTY; without even the implied warranty of");
    println!("MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the");
    println!("GNU Affero General Public License for more details.\n");
    println!("You should have received a copy of the GNU Affero General Public License");
    println!("along with this program.  If not, see https://www.gnu.org/licenses/.");
    println!("----------------------------------------------------------------------------\n");
}

fn print_external_librarys() {
    println!("This software uses external libraries:");
    println!("dbus-rs v0.9.5 (Apache-2.0/MIT)");
    println!("\n");
}
