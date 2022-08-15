import type Database from "tauri-plugin-sqlite-api";

export enum Tables {
  Collections = "collections",
  Sites = "sites",
  Settings = "settings",
  Images = "images",
}

export class BaseDataService {
  protected db: Database;
  initialized: Promise<void>;

  constructor(db: Database) {
    this.db = db;

    this.initialized = this.init();
  }

  private async init() {
    // always
    await this.createTables();
  }

  private createTables() {
    return this.db.execute(/*sql*/ `
      CREATE TABLE IF NOT EXISTS ${Tables.Settings} (
        settings_key VARCHAR(255),
        settings_json TEXT
      );

      CREATE TABLE IF NOT EXISTS ${Tables.Sites} (
        site_id VARCHAR(255),
        site_name VARCHAR(255)
      );

      CREATE TABLE IF NOT EXISTS ${Tables.Collections} (
        collection_id VARCHAR(255),
        collection_name VARCHAR(255),
        collection_key VARCHAR(255),
        site_id VARCHAR(255),
        FOREIGN KEY(site_id) REFERENCES sites(site_id)
      );

      CREATE TABLE IF NOT EXISTS ${Tables.Images} (
        image_id VARCHAR(255),
        image_type VARCHAR(255),
        uploaded BOOLEAN,
        collection_id VARCHAR(255),
        original_file_name VARCHAR(255),
        source_url VARCHAR(255),
        FOREIGN KEY(collection_id) REFERENCES collections(collection_id)
      );
    `);
  }

  dropTables() {
    return this.db.execute(/*sql*/ `
      DROP TABLE IF EXISTS ${Tables.Settings};
      DROP TABLE IF EXISTS ${Tables.Images};
      DROP TABLE IF EXISTS ${Tables.Collections};
      DROP TABLE IF EXISTS ${Tables.Sites};
    `);
  }
}
