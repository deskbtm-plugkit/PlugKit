import { useRef, useState } from 'react';
import React from 'react';
import { appWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';

function App() {
  const [count, setCount] = useState(0);
  const ref = useRef<HTMLDivElement>(null);

  return (
    <div className="App">
      <div ref={ref}></div>
      <button
        onClick={async () => {
          emit('demo1', {
            theMessage: 'Tauri is awesome!',
          });
          // await appWindow.emit('state-changed', {
          //   loggedIn: true,
          //   token: 'authToken',
          // });
        }}
      >
        Plugin
      </button>

      <div
        className="box"
        style={{
          width: 300,
          height: 300,
          background: 'green',
        }}
      ></div>
    </div>
  );
}

export default App;
