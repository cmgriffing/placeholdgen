import type { SiteImage } from "../../types/SiteImage";
import { BaseDataService, Tables } from "./_base";

export class ImagesService extends BaseDataService {
  async createImage(image: SiteImage) {
    const keys = [];
    const values = [];

    Object.entries(image).forEach(([key, value]) => {
      keys.push(key);
      values.push(value);
    });

    const escapedValues = values.map((value) => {
      if (typeof value === "string") {
        return `"${value.replaceAll('"', '\\"')}"`;
      } else {
        return value;
      }
    });

    const query = /*sql*/ `
    INSERT
    INTO ${Tables.Images}
    (${keys.join(", ")})
    VALUES (${escapedValues.join(", ")})
  `;

    console.log({ query });

    const createResult = await this.db.execute(query);

    console.log({ createResult });
  }
  async getCollectionImages(collectionId: string) {
    return this.db.select<SiteImage[]>(/*sql*/ `
      SELECT *
      FROM ${Tables.Images}
      WHERE collection_id = "${collectionId}"
    `);
  }
  async getCollectionImage(collectionId: string, imageId: string) {
    return this.db.select<SiteImage[]>(/*sql*/ `
      SELECT *
      FROM ${Tables.Images}
      WHERE image_id = "${imageId}"
      AND collection_id = "${collectionId}"
      LIMIT 1
    `);
  }
  async updateSite(image: SiteImage) {
    const formattedEntries = Object.entries(image)
      .map(([key, value]) => {
        if (typeof value === "string") {
          value = `"${value.replaceAll('"', '\\"')}"`;
        }
        return `${key} = ${value}`;
      })
      .join(", ");

    return this.db.execute(/*sql*/ `
      UPDATE ${Tables.Images}
      SET ${formattedEntries}
      WHERE image_id = "${image.image_id}"
    `);
  }
  async deleteImage(image: SiteImage) {
    return this.db.execute(/*sql*/ `
      DELETE
      FROM ${Tables.Images}
      WHERE image_id = "${image.image_id}"
    `);
  }
}
