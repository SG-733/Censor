import React, { useState } from 'react';
import { Connection, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';

const NETWORK_URL = 'https://api.devnet.solana.com';
const PROGRAM_ID = 'your_program_id'; // Replace with your actual program ID

const connection = new Connection(NETWORK_URL, 'confirmed');

function App() {
  const [recipient, setRecipient] = useState('');
  const [content, setContent] = useState('');
  const [status, setStatus] = useState('');

  const handleSendMessage = async () => {
    setStatus('Sending message...');
    try {
      const toPublicKey = new PublicKey(recipient);
      const data = Buffer.from(content);
      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: PublicKey.default, isSigner: true, isWritable: true },
          { pubkey: toPublicKey, isSigner: false, isWritable: true }
        ],
        programId: new PublicKey(PROGRAM_ID),
        data,
      });

      const transaction = new Transaction().add(instruction);
      const signature = await window.solana.signAndSendTransaction(transaction);
      setStatus(`Message sent successfully! Signature: ${signature}`);
    } catch (error) {
      console.error('Error sending message:', error);
      setStatus('Failed to send message.');
    }
  };

  return (
    <div>
      <h1>Solana Message Service</h1>
      <div>
        <label>Recipient:</label>
        <input type="text" value={recipient} onChange={e => setRecipient(e.target.value)} />
      </div>
      <div>
        <label>Message:</label>
        <textarea value={content} onChange={e => setContent(e.target.value)} />
      </div>
      <button onClick={handleSendMessage}>Send Message</button>
      {status && <p>{status}</p>}
    </div>
  );
}

export default App;
