import type { Settings } from "./../../types/Settings";
import { BaseDataService, Tables } from "./_base";

const SETTINGS_KEY = "SETTINGS_KEY";

export class SettingsService extends BaseDataService {
  async getSettings() {
    const settingsResult = await this.db.select<{
      settings_key: string;
      settings_json: string;
    }>(/*sql*/ `
      SELECT *
      FROM ${Tables.Settings}
      WHERE settings_key = "${SETTINGS_KEY}"
    `);

    return JSON.parse(settingsResult.settings_json);
  }
  async updateSettings(settings: Settings) {
    return this.db.execute(/*sql*/ `
      UPDATE ${Tables.Settings}
      SET settings_json = "${JSON.stringify(settings)}"
      WHERE settings_key = "${SETTINGS_KEY}"
    `);
  }
}
