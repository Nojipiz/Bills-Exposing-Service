import * as express from 'express';
import axios from 'axios';

const app = express()
const PORT: number = 3000;

app.get('/get_bill', (req: express.Request, res: express.Response) => {
  const billId: number = parseInt(req.header('bill_id')!);
  const billPeriod: number = parseInt(req.header('period')!);
  if (!billId || !billPeriod) res.header('Error', 'Not numeric values are alowed').status(400).end();
  const query: string = `http://127.0.0.1:8080/bills/${billPeriod}/${billId}`;
  axios.get(query).then(response => {
  })
})

app.listen(3000, () => {
  console.log(`Express server started in port ${PORT}`)
})
