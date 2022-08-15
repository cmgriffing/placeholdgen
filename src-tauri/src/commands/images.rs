use glob::glob;
use std::path::Path;
use std::{ffi::OsStr, fs::create_dir_all};

use image_convert::{to_png, ImageResource, MagickError, PNGConfig};

use crate::types::structs::SiteImage;

#[tauri::command]
pub fn add_local_image(
    app_handle: tauri::AppHandle,
    local_image_path: String,
    site_id: String,
    collection_id: String,
    image_id: String,
) {
    let local_image_path_path = Path::new(&local_image_path);

    // guard local_image_path existing
    let local_image_exists = local_image_path_path.exists();

    if !local_image_exists {
        println!("local image does not exist: {local_image_path}");
        return;
    }

    let input = ImageResource::from_path(local_image_path_path);

    let convert_result = convert_image(app_handle, collection_id, image_id, input);

    println!(
        "convert_result: is_err:{} is_ok:{}",
        convert_result.is_err(),
        convert_result.is_ok()
    );

    if convert_result.is_err() {
        println!("Error: {}", convert_result.unwrap_err());
    }
}

#[tauri::command]
pub fn get_all_images(app_handle: tauri::AppHandle) -> Vec<String> {
    let resource_dir = app_handle.path_resolver().resource_dir().unwrap();

    let resource_dir_string = resource_dir.as_os_str().to_str().unwrap();

    let images = glob(&format!("{resource_dir_string}/**/*.{{png,gif}}")).unwrap();

    return images
        .map(|image_path| image_path.unwrap().to_string_lossy().to_string())
        .collect();
}

#[tauri::command]
pub fn add_remote_image(app_handle: tauri::AppHandle, remote_image: SiteImage) -> bool {
    let image_fetch_result = ureq::get(&remote_image.source_url).call();

    match image_fetch_result {
        Ok(result) => {
            let image_resource = ImageResource::from_reader(result.into_reader());

            if image_resource.is_err() {
                return false;
            }

            let convert_result = convert_image(
                app_handle,
                remote_image.collection_id,
                remote_image.image_id,
                image_resource.unwrap(),
            );

            println!(
                "convert_result: is_err:{} is_ok:{}",
                convert_result.is_err(),
                convert_result.is_ok()
            );

            if convert_result.is_err() {
                println!("Error: {}", convert_result.unwrap_err());
            }

            true
        }
        Err(error) => {
            println!("Error fetching image: {:?}", error);
            false
        }
    }
}

#[tauri::command]
pub fn search_images(app_handle: tauri::AppHandle, search_query: String) -> bool {
    let path = format!(
        "https://api.imgur.com/3/gallery/search/top/?q={}'",
        search_query
    );
    let image_fetch_result = ureq::get(&path)
        .set("Authorization", &format!("Client-ID {}", "572f73f3aca31fb"))
        .call();

    match image_fetch_result {
        Ok(result) => {
            let image_resource = ImageResource::from_reader(result.into_reader());

            if image_resource.is_err() {
                return false;
            }

            let convert_result = convert_image(
                app_handle,
                remote_image.collection_id,
                remote_image.image_id,
                image_resource.unwrap(),
            );

            println!(
                "convert_result: is_err:{} is_ok:{}",
                convert_result.is_err(),
                convert_result.is_ok()
            );

            if convert_result.is_err() {
                println!("Error: {}", convert_result.unwrap_err());
            }

            true
        }
        Err(error) => {
            println!("Error fetching image: {:?}", error);
            false
        }
    }
}

fn convert_image(
    app_handle: tauri::AppHandle,
    collection_id: String,
    image_id: String,
    input: ImageResource,
) -> Result<(), MagickError> {
    let copy_directory_path_string = format!(
        "{}/collections/{}",
        app_handle
            .path_resolver()
            .resource_dir()
            .ok_or("Error getting resource_dir")?
            .as_os_str()
            .to_str()
            .ok_or("Error converting resource_dir to str")?,
        collection_id.clone()
    );

    let copy_directory_path = Path::new(&copy_directory_path_string);

    if !copy_directory_path.exists() {
        let create_result = create_dir_all(copy_directory_path);
        if create_result.is_err() {
            println!("SOMETHING HAS GONE TERRIBLY WRONG");
        }
    }

    let copy_image_path_string = format!(
        "{}/{image_id}.png",
        copy_directory_path
            .as_os_str()
            .to_str()
            .ok_or("Error converting copy_directory_path to str")?
    );

    let copy_image_path = Path::new(&copy_image_path_string);

    let mut config = PNGConfig::new();

    config.width = 3840;

    let mut output = ImageResource::from_path(copy_image_path.clone());

    to_png(&mut output, &input, &config)
}
