import axios from 'axios';

export async function getServerData() {
  let config = {
    headers: {
      "bill_id": "123",
      "period": "2020",
    }
  }
  const serverResponse = await axios.get('http://localhost:3000/get_bill', config);
  const responseStatus = serverResponse.status;
  switch (responseStatus) {
    case 400 || 503:
      throw new Error(responseStatus);
    case 200:
      return dataToPDFFile(serverResponse.data);
    default:
      break;
  }
}

export async function dataToPDFFile(data) {
  const fileUrl = `data:application/pdf;base64,${data}`;
  return fileUrl;
}

