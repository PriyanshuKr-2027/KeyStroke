import React from 'react';
import ReactDOM from 'react-dom/client';
import { Palette } from './Palette';

ReactDOM.createRoot(document.getElementById('palette-root') as HTMLElement).render(
  <React.StrictMode>
    <Palette />
  </React.StrictMode>
);
