use glob::glob;
use pickledb::PickleDb;
use std::fs::{self, create_dir};
use std::path::Path;
use std::sync::Mutex;
use tauri::Manager;

use image_convert::{to_png, ImageResource, PNGConfig};

use crate::types::structs::{Collection, SiteImage};
use crate::types::{constants::DB_KEY_APP_STATE, structs::AppState};

#[tauri::command]
pub fn add_local_image(
    app_handle: tauri::AppHandle,
    local_image_path: String,
    site_id: String,
    collection_id: String,
    image_id: String,
    db: tauri::State<'_, Mutex<PickleDb>>,
) -> AppState {
    let mut new_db = db.lock().unwrap();

    let mut app_state = new_db.get::<AppState>(DB_KEY_APP_STATE).unwrap();

    let local_image_path_path = Path::new(&local_image_path);

    // guard local_image_path existing
    let local_image_exists = local_image_path_path.exists();

    if !local_image_exists {
        println!("local image does not exist: {local_image_path}");
        return app_state;
    }

    println!("SITE_ID: {}", site_id);

    let mut site = app_state
        .sites
        .iter()
        .find(|inner_site| inner_site.site_id == site_id)
        .unwrap()
        .clone();

    let mut collections = site.collections.clone();

    let collection_maybe = site.collections.iter().find(|collection| {
        println!("collection.collection_id, {}", collection.collection_id);
        collection.collection_id == collection_id
    });

    let collection = match collection_maybe.is_none() {
        true => {
            println!("app_state after creating collection, {:?}", app_state);
            let new_collection = Collection {
                collection_id: collection_id.clone(),
                name: "New Collection".to_string(),
                images: vec![],
            };

            collections.push(new_collection.clone());

            new_collection
        }
        false => collection_maybe.unwrap().clone(),
    };

    collections.push(collection.clone());

    collections = collections
        .iter()
        .map(|inner_collection| {
            let cloned_collection = collection.clone();
            let cloned_inner_collection = inner_collection.clone();
            match cloned_collection.collection_id == cloned_inner_collection.collection_id {
                true => cloned_collection,
                false => cloned_inner_collection,
            }
        })
        .collect();

    site.collections = collections;

    let mut sites = app_state.sites.clone();

    sites = sites
        .iter()
        .map(|inner_site| {
            let cloned_site = site.clone();
            let cloned_inner_site = inner_site.clone();
            match cloned_site.site_id == cloned_inner_site.site_id {
                true => cloned_site,
                false => cloned_inner_site,
            }
        })
        .collect();

    app_state.sites = sites;

    let set_result = new_db
        .set::<AppState>(DB_KEY_APP_STATE, &app_state)
        .unwrap();

    // let image_result = File::open(local_image_path).unwrap();

    let copy_directory_path_string = format!(
        "{}/collections/{}",
        app_handle
            .path_resolver()
            .resource_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
        collection_id.clone()
    );

    let copy_directory_path = Path::new(&copy_directory_path_string);

    if !copy_directory_path.exists() {
        let create_result = create_dir(copy_directory_path);
        if create_result.is_err() {
            println!("SOMETHING HAS GONE TERRIBLY WRONG");
        }
    }

    println!(
        "copy_directory_path: {}",
        copy_directory_path.as_os_str().to_str().unwrap()
    );

    let copy_image_path_string = format!(
        "{}/{image_id}.png",
        copy_directory_path.as_os_str().to_str().unwrap()
    );

    let copy_image_path = Path::new(&copy_directory_path_string);

    println!("copy_image_path: {copy_image_path_string}");

    let mut config = PNGConfig::new();

    config.width = 3840;

    let input = ImageResource::from_path(local_image_path_path);

    let mut output = ImageResource::from_path(copy_image_path);

    let convert_result = to_png(&mut output, &input, &config);

    println!(
        "convert_result: is_err:{} is_ok:{}",
        convert_result.is_err(),
        convert_result.is_ok()
    );

    let new_app_state = new_db.get::<AppState>(DB_KEY_APP_STATE).unwrap();

    app_handle
        .emit_all("app_state_update", new_app_state.clone())
        .unwrap();

    new_app_state
}

#[tauri::command]
pub fn get_all_images(app_handle: tauri::AppHandle) -> Vec<SiteImage> {
    let resource_dir = app_handle.path_resolver().resource_dir();

    // LEAVING OFF: JUST PASTED
    for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    return vec![];
}
