import { Component } from 'react';
import { getServerData } from '../api.js';
import styles from './bills.module.css';

class Bills extends Component {

  constructor(props) {
    super(props);
    this.state = {
      pdfView: undefined,
    }
  }

  async getBill() {
    const data = await getServerData();
    console.log(data);
    return (
      // <embed id="plugin" type="application/x-google-chrome-pdf" original-url={data} />
      <embed type="application/x-google-chrome-pdf" src="/home/nojipiz/Downloads/test/FETU123.pdf" />
    )
  }

  addBillPdf() {
    const element = this.getBill();
    this.setState({ pdfView: element });
  }

  render() {
    return (
      <div className={styles.mainContainer}>
        <img src='/images/Factura.png' className={styles.instructionsImg} />
        <div className={styles.billNumberContainer}>
          <input type='number' />
          <button onClick={this.addBillPdf}>Descargar PDF</button>
          <embed type="application/x-google-chrome-pdf" src="/home/nojipiz/Downloads/test/FETU123.pdf" />
        </div>
      </div >
    )
  }

}

export default Bills; 
