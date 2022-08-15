import type { Site } from "./../../types/Site";
import { BaseDataService, Tables } from "./_base";

export class SitesService extends BaseDataService {
  async createSite(site: Site) {
    const keys = [];
    const values = [];

    Object.entries(site).forEach(([key, value]) => {
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
    INTO ${Tables.Sites}
    (${keys.join(", ")})
    VALUES (${escapedValues.join(", ")})
  `;

    console.log({ query });

    const createResult = await this.db.execute(query);

    console.log({ createResult });
  }
  async getSites() {
    return this.db.select<Site[]>(/*sql*/ `
      SELECT *
      FROM ${Tables.Sites}
    `);
  }
  async getSite(siteId: string) {
    return this.db.select<Site>(/*sql*/ `
      SELECT *
      FROM ${Tables.Sites}
      WHERE site_id = "${siteId}"
      LIMIT 1
    `);
  }
  async updateSite(site: Site) {
    const formattedEntries = Object.entries(site)
      .map(([key, value]) => {
        if (typeof value === "string") {
          value = `"${value.replaceAll('"', '\\"')}"`;
        }
        return `${key} = ${value}`;
      })
      .join(", ");

    return this.db.execute(/*sql*/ `
      UPDATE ${Tables.Sites}
      SET ${formattedEntries}
      WHERE site_id = "${site.site_id}"
    `);
  }
  async deleteSite(site: Site) {
    return this.db.execute(/*sql*/ `
      DELETE
      FROM ${Tables.Sites}
      WHERE site_id = "${site.site_id}"
    `);
  }
}
