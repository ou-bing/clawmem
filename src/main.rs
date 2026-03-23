use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn handle_event(event: Event) {
    match event.kind {
        EventKind::Create(_) => println!("文件创建: {:?}", event.paths),
        EventKind::Modify(_) => println!("文件修改: {:?}", event.paths),
        EventKind::Remove(_) => println!("文件删除: {:?}", event.paths),
        _ => {}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取监听目录，默认当前目录
    let watch_path = env::var("WATCH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    let config = Config::default()
        .with_poll_interval(Duration::from_secs(1));

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                handle_event(event);
            }
        },
        config,
    )?;

    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    println!("开始监听文件夹: {:?}", watch_path);
    println!("按 Ctrl+C 退出\n");

    // 保持程序运行
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
