// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() -> String {
    println!("I was invoked from JS!");
    return String::from("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test] //由此判断这是一个测试函数
    fn it_works() {
        // let results = get_images();
        // assert_eq!(2, results.len())
    }
}
