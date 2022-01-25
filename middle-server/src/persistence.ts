import { readFile, writeFile } from 'fs/promises';

const CONFIG_FILE_PATH: string = './resources/config.json';

async function getSavedConfig(): Promise<Config> {
  const savedData = await readFile(CONFIG_FILE_PATH);
  return JSON.parse(savedData.toString());
}

async function saveConfig(data: string): Promise<void> {
  const lastSavedConfig = await getSavedConfig();
  const dataToSave: Config = { address: data, auth: lastSavedConfig.auth };
  await writeFile(CONFIG_FILE_PATH, JSON.stringify(dataToSave));
}

interface Config {
  address: string;
  auth: string;
}

export { getSavedConfig, saveConfig, Config };


