use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

pub fn set_file_permissions_numeric(file_path: &str, mode: u32) -> Result<(), std::io::Error> {
    // 获取文件的元数据
    let metadata = fs::metadata(file_path)?;

    // 获取当前权限并创建一个可修改的副本
    let mut perms = metadata.permissions();

    perms.set_mode(mode);

    // 将新权限应用到文件
    fs::set_permissions(file_path, perms)?;

    Ok(())
}

pub fn inotify_init(path: &str) -> inotify::Inotify {
    let inotify = inotify::Inotify::init().unwrap();
    inotify
        .watches()
        .add(path, inotify::WatchMask::MODIFY)
        .unwrap();
    inotify
}

pub fn inotify_blockage(inotify: &mut inotify::Inotify) {
    let mut buffer = [0u8; 4096];
    loop {
        match inotify.read_events(&mut buffer) {
            Ok(events) => {
                #[allow(clippy::never_loop)]
                for _ in events {
                    break;
                }
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                panic!("读取 inotify 事件失败: {:?}", e);
            }
        }
    }
}

pub fn monitor_screen_status() -> bool {
    let result = Command::new("sh")
        .arg("-c")
        .arg("dumpsys power | grep -E \"mHoldingDisplaySuspendBlocker|mScreenOn\"")
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&result.stdout);
    output.contains("true")
}

pub fn get_now_top_window_pkg_name() -> Vec<String> {

    let result = Command::new("sh")
        .arg("-c")
        .arg(
            "dumpsys window | grep -E 'mCurrentFocus=Window|mFocusedApp'"
        )
        .output()
        .expect("Failed to execute command");


    let output =
        String::from_utf8_lossy(&result.stdout);



    let mut packages: Vec<String> = Vec::new();



    for line in output.lines() {


        // 示例:
        // mCurrentFocus=Window{xxxx u0 com.netease.party/com.xxx.MainActivity}


        if let Some(pos) = line.find(" u0 ") {


            let info = &line[pos + 4..];


            if let Some(pkg) = info.split('/').next() {


                let pkg = pkg
                    .replace("}", "")
                    .trim()
                    .to_string();



                if !pkg.is_empty()
                    && !packages.contains(&pkg)
                {

                    packages.push(pkg);

                }

            }

        }

    }


    packages
            }
