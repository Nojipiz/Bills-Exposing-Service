import { readFile, writeFile } from 'fs/promises';

const CONFIG_FILE_PATH: string = './resources/config.json';

async function getSavedAddress(): Promise<Config> {
  const savedData = await readFile(CONFIG_FILE_PATH);
  return JSON.parse(savedData.toString());
}

async function saveAddress(data: string): Promise<void> {
  const dataToSave: Config = { address: data };
  await writeFile(CONFIG_FILE_PATH, JSON.stringify(dataToSave));
}

interface Config {
  address: string;
}

export { getSavedAddress, saveAddress, Config };


