import * as express from 'express';
import axios from 'axios';
import { Config, getSavedConfig, saveConfig } from './persistence';

const app = express()
const PORT: number = 3000;
const BILLS_END_POINT: string = "/bills/";

app.get('/get_bill', async (req: express.Request, res: express.Response) => {
  const billId: number = parseInt(req.header('bill_id')!);
  const billPeriod: number = parseInt(req.header('period')!);
  if (!billId || !billPeriod) res.send('Error Only numeric values are alowed').status(400);
  const configContent: Config = await getSavedConfig();
  const query: string = `http://${configContent.address}${BILLS_END_POINT}${billPeriod}/${billId}`;
  axios.get(query).then(response => {
    res.status(200).send(response.data);
  }).catch(() => {
    res.status(503).end();
  });
})

app.post('/set_server_address', async (req: express.Request, res: express.Response) => {
  const address: string = req.header('address') || '';
  const auth: string = req.header('auth') || '';
  if (!address) return res.status(503).end();
  const lastSavedConfig = await getSavedConfig();
  if (lastSavedConfig.auth !== auth) return res.status(401).end();
  await saveConfig(address);
  res.status(200).send(address);
})

app.listen(3000, () => {
  console.log(`Express server started in port ${PORT}`)
})
