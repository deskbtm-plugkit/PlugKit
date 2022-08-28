// import { invoke } from '@tauri-apps/api/tauri';
import { useRef, useState } from 'react';
import React from 'react';
import { appWindow } from '@tauri-apps/api/window';
import { homeDir, resolve } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { invoke } from '@tauri-apps/api';
import './App.css';
import { Route, BrowserRouter, Routes, useNavigate } from 'react-router-dom';
import {
  isPermissionGranted,
  sendNotification,
} from '@tauri-apps/api/notification';

const children: React.ReactNode[] = [];

appWindow.listen<string>('state-changed', (event) => {
  console.log(`Got error: `, event);
});

const handleChange = (value: string) => {
  console.log(`selected ${value}`);
};

function Page1() {
  const [count, setCount] = useState(0);
  const [url, setUrl] = useState<string | null>(null);
  const ref = useRef<HTMLDivElement>(null);

  const navigate = useNavigate();
  // appWindow.show();

  return (
    <div className="App">
      <div data-tauri-drag-region className="titlebar">
        <div className="titlebar-button" id="titlebar-minimize">
          <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-maximize">
          <img
            src="https://api.iconify.design/mdi:window-maximize.svg"
            alt="maximize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-close">
          <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </div>
      </div>
      {/* <button
        onClick={async () => {
          const r = await invoke('my_custom_command');
          console.log(r);
        }}
      >
        click
      </button> */}
      <div ref={ref}></div>

      <button
        onClick={() => {
          navigate('/page2');
        }}
      >
        Demo
      </button>
      {/* <iframe src="" frameBorder="0"></iframe> */}
      <button
        onClick={async () => {
          const permissionGranted = await isPermissionGranted();
          console.log(permissionGranted);

          // sendNotification('Tauri is awesome!');
          sendNotification({
            title: 'TAURI',
            body: 'Tauri is awesome!',
            icon: '/vite.svg',
          });
          // console.log('=======');
          // const w = new WebviewWindow('Setting', {
          //   url: 'http://localhost:5173/setting/index.html',
          // });
          // console.log(w);
        }}
      >
        new Window
      </button>
      <button
        onClick={() => {
          // "geolocation" | "notifications" | "persistent-storage" | "push" | "screen-wake-lock" | "xr-spatial-tracking";
          navigator.permissions.query({ name: 'geolocation' }).then((e) => {
            console.log(e);
          });
          navigator.permissions.query({ name: 'notifications' }).then((e) => {
            console.log(e);
          });
          navigator.permissions
            .query({ name: 'persistent-storage' })
            .then((e) => {
              console.log(e);
            });
          navigator.permissions.query({ name: 'push' }).then((e) => {
            console.log(e);
          });

          navigator.permissions
            .query({ name: 'screen-wake-lock' })
            .then((e) => {
              console.log(e);
            });

          navigator.clipboard.read().then((e) => {
            console.log(e);
          });
          // appWindow.hide();
        }}
      >
        Clipboard
      </button>
      <button
        onClick={() => {
          appWindow.hide();
        }}
      >
        Hide
      </button>
      <button
        onClick={async () => {
          // import('http://osd.deskbtm.com/case.js' as any).then((e) => {
          //   console.log(e);
          // });

          await appWindow.emit('state-changed', {
            loggedIn: true,
            token: 'authToken',
          });

          // const webview = new WebviewWindow('theUniqueLabel', {
          //   url: 'http://osd.deskbtm.com/case.html',
          // });

          const home = await homeDir();
          const a = await resolve(
            home,
            'AbyssProject/abyss/app/public/case.html',
          );
          console.log(a);

          const htmlUri = convertFileSrc(a);

          setUrl(htmlUri);
          console.log(htmlUri);
          await appWindow.setFullscreen(true);
          // await appWindow.maximize();

          // await appWindow.setSize()

          setTimeout(async () => {
            const r = await invoke('my_custom_command');
          }, 4000);

          // console.log(ref.current);
          // ref.current!.innerHTML =
          //   '<iframe id="inlineFrameExample" title="Inline Frame Example" width="300" height="200" src="http://www.baidu.com"></iframe>';
          // console.log(r);
        }}
      >
        Plugin
      </button>

      <img src="/vite.svg" />

      <iframe
        id="inlineFrameExample"
        title="Inline Frame Example"
        width="300"
        height="200"
        style={{ border: 'none' }}
        src={url!}
      ></iframe>

      <div
        className="box"
        style={{
          width: 300,
          height: 300,
          background: 'pink',
        }}
      ></div>
      {Array.from({ length: 100 }).map(() => {
        return <br />;
      })}
    </div>
  );
}

function Page2() {
  return <div>Page2</div>;
}

function App() {
  console.log('------------------');
  return (
    <BrowserRouter basename="/app/src">
      <Routes>
        <Route path="/">
          <Route index element={<Page1 />} />
          <Route path="/page2" element={<Page2 />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
