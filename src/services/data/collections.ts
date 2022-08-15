import type { Collection } from "./../../types/Collection";
import { BaseDataService, Tables } from "./_base";
export class CollectionsService extends BaseDataService {
  createCollection(siteId: string, collection: Collection) {
    const keys = [];
    const values = [];

    Object.entries(collection).forEach(([key, value]) => {
      keys.push(key);
      values.push(value);
    });

    keys.push("site_id");
    values.push(siteId);

    const escapedValues = values.map((value) => {
      if (typeof value === "string") {
        value = `"${value.replaceAll('"', '\\"')}"`;
      }
      return value;
    });

    return this.db.execute(/*sql*/ `
      INSERT
      INTO ${Tables.Collections}
      (${keys.join(", ")})
      VALUES (${escapedValues.join(", ")})
    `);
  }
  getCollections(siteId: string) {
    return this.db.select<Collection[]>(/*sql*/ `
      SELECT *
      FROM ${Tables.Collections}
      WHERE site_id = "${siteId}"
    `);
  }
  getCollection(siteId: string, collectionId: string) {
    return this.db.select<Collection[]>(/*sql*/ `
      SELECT *
      FROM ${Tables.Collections}
      WHERE site_id = "${siteId}" AND collection_id = "${collectionId}"
    `);
  }
  updateCollection(siteId: string, collection: Collection) {
    const formattedEntries = Object.entries(collection)
      .map(([key, value]) => {
        if (typeof value === "string") {
          value = `"${value.replaceAll('"', '\\"')}"`;
        }
        return `${key} = ${value}`;
      })
      .join(", ");

    return this.db.execute(/*sql*/ `
      UPDATE ${Tables.Collections}
      SET ${formattedEntries}
      WHERE site_id = "${siteId}" AND collection_id = "${collection.collection_id}"
    `);
  }
  deleteCollection(siteId: string, collection: Collection) {
    return this.db.execute(/*sql*/ `
      DELETE
      FROM ${Tables.Collections}
      WHERE site_id = "${siteId}" AND collection_id = "${collection.collection_id}"
    `);
  }
}
