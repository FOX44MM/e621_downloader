/*
 * Copyright (c) 2022 McSib
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate log;

use std::env::consts::{
    ARCH, DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_EXTENSION, EXE_SUFFIX, FAMILY, OS,
};
use std::fs::File;

use crate::e621::io::tag::{Group, Tag, TagSearchType, TagType};
use crate::program::Program;
use clap::Parser;
use console::Term;
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};

mod e621;
mod program;

/// 命令行参数 默认使用配置文件的形式，如果有其它参数，则使用默认的形式
#[derive(Parser, Debug)]
struct Args {
    // todo tag文件
    #[arg(long, required = false)]
    tag: Option<String>,

    // 艺术家
    #[arg(long, required = false, short = 'A', num_args = 0..)]
    artist: Option<Vec<String>>,

    // 池
    #[arg(long, required = false, short = 'S', num_args = 0..)]
    pool: Option<Vec<i64>>,
}

impl Args {
    fn is_null(&self) -> bool {
        // info!("{:?}", self);
        self.tag.is_none() && self.artist.is_none() && self.pool.is_none()
    }

    fn to_groups(&self) -> Vec<Group> {
        // 将命令行参数导出为Group数组
        let mut groups: Vec<Group> = Vec::new();

        let mut artist_group = Group::new(String::from("artists"));
        for artist in self.artist.clone().unwrap_or_default().iter_mut() {
            artist_group
                .tags
                .push(Tag::new(artist, TagSearchType::Special, TagType::Artist));
        }

        let mut pool_group = Group::new(String::from("pool"));
        for pool in self.pool.clone().unwrap_or_default() {
            
            pool_group.tags.push(Tag::new(&*pool.to_string(), TagSearchType::General, TagType::Pool));
        }
        
        // let mut set_group = Group::new(String::from("set"));

        groups.push(artist_group);
        groups.push(pool_group);
        // groups.push(set_group);
        groups
    }
}

fn main() {
    initialize_logger();
    log_system_information();
    Term::stdout().set_title("e621 downloader"); // 将命令行标题设置为e621 downloader

    let args = Args::parse();
    let program = Program::new();

    info!("{}", args.is_null());

    if args.is_null() {
        program.run().expect("出错");
        // info!("正常运行程序")
    } else {
        // info!("运行命令行程序");
        program.run_in_arg(&*args.to_groups(), false);
    }
}

/// Initializes the logger with preset filtering.
fn initialize_logger() {
    let mut config = ConfigBuilder::new();
    config.add_filter_allow_str("e621_downloader");

    CombinedLogger::init(vec![
        // 终端Info
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // 日志文件
        WriteLogger::new(
            LevelFilter::max(),
            config.build(),
            File::create("e621_downloader.log").unwrap(),
        ),
    ])
    .unwrap();
}

/// Logs important information about the system being used.
fn log_system_information() {
    trace!("Printing system information out into log for debug purposes...");
    trace!("ARCH:           \"{}\"", ARCH);
    trace!("DLL_EXTENSION:  \"{}\"", DLL_EXTENSION);
    trace!("DLL_PREFIX:     \"{}\"", DLL_PREFIX);
    trace!("DLL_SUFFIX:     \"{}\"", DLL_SUFFIX);
    trace!("EXE_EXTENSION:  \"{}\"", EXE_EXTENSION);
    trace!("EXE_SUFFIX:     \"{}\"", EXE_SUFFIX);
    trace!("FAMILY:         \"{}\"", FAMILY);
    trace!("OS:             \"{}\"", OS);
}
