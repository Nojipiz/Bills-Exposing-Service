import * as express from 'express';
import axios from 'axios';

const app = express()
const PORT: number = 3000;
const FILES_SERVER_URL: String = "http://127.0.0.1:8080/bills/"

app.get('/get_bill', (req: express.Request, res: express.Response) => {
  const billId: number = parseInt(req.header('bill_id')!);
  const billPeriod: number = parseInt(req.header('period')!);
  if (!billId || !billPeriod) res.send('Error Only numeric values are alowed').status(400);
  const query: string = `${FILES_SERVER_URL}${billPeriod}/${billId}`;
  axios.get(query).then(response => {
    res.status(200).send(response.data);
  }).catch(() => {
    res.status(503).end();
  });
})

app.post('/set_server_address', (req: express.Request, res: express.Response) => {
  console.log(req.header('address'));
})

app.listen(3000, () => {
  console.log(`Express server started in port ${PORT}`)
})
