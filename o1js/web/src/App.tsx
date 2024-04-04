import React, { useEffect, useState } from 'react';
import { bcs } from '@mysten/bcs';
import { z } from 'zod';

import { verify } from './verifier';

function App() {
  const [jsonData, setJsonData] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const response = await fetch('../../../signature_poc.json');
        const signatureData = await response.text();
        setJsonData(signatureData);

        await verify(signatureData);

      } catch (error) {
        console.error('Error fetching JSON data:', error);
      }
    })();
  }, []);

  return (
    <div>
      <h1>Signature POC</h1>
      {jsonData ? <pre>{jsonData}</pre> : <p>Loading JSON data...</p>}
    </div>
  );
}

export default App;
